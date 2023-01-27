use crate::prelude::*;

type Depth = usize; // TODO: move to types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZorbistKey(u64);

impl ZorbistKey {
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }
    
    #[inline(always)]
    pub const fn index<const BASE: usize>(self) -> usize {
        (self.0 as usize) % BASE
    }

    #[inline(always)]
    pub const fn empty(self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    pub const fn xor(self, change: u64) -> Self {
        Self(self.0 ^ change)
    }

    #[inline(always)]
    pub fn mut_xor(&mut self, change: u64) {
        self.0 ^= change;
    }
}

// PERF: try to remove unused entries
pub const PIECE_SQUARE_TO_HASH: [[u64; 64]; 16] = [
    // PieceNone - not used
    [0xabfc246d1d401815, 0x218ed69adc596bc1, 0xeec58202663341f4, 0xb4a4238f90adaf76, 0x17e9af1153d05f3b, 0xbfcfa30cfbf2beee, 0xc032fabbffc9ec51, 0x12bea088eb2fcc3e, 0x36dca758acde41f9, 0xdc86d0db35ea2dd6, 0x4ceb77de0cfa7055, 0xd306be66e7f58dd5, 0xb975f9a90ca7ef67, 0x51fd0633eab1f8ad, 0x274062aaa0fdb71d, 0xaa865cdb4cde77d2, 0xcce9ffa6f00c9e0e, 0x24815b17cf7f5a30, 0x8c24cab67ecbb7cd, 0xdea3c445ce321b4c, 0xa4324dfa570ce246, 0x5c9312ec1b3d2b25, 0xcadcd43bbb1fdc44, 0xac34f05a9250f98d, 0xb4cc2766ffbad073, 0xc197613189359469, 0xfbbd27a7e227cab9, 0xa11c1a51282b2342, 0xdbee0d66caffdc6, 0xabdca1aa602e4aca, 0x907b30fd3661c771, 0xacfbbef75f113a08, 0x2eac035cc29d5a21, 0x96b33a7a06e63730, 0xb3d92429f48de6a1, 0xb53a34ac247de807, 0xfc88cdb0ab783595, 0xf4471b3f9e2052b7, 0x7efb9af04f76e579, 0xb46c9b6d6d33d470, 0xe256bafb10755a51, 0x4cefbb8d5a24c903, 0x54a6e022b3bb086e, 0x5395ba034e0699a4, 0xca9e20e8c5b39b33, 0xa58743d97c76d584, 0x6b3b39e2a1638566, 0x7b47913299e6d6bc, 0x486f0f5b39617c12, 0x4f44d3908d899257, 0xd1f9347812b94007, 0x330f297484bef2bc, 0x255658b8690d22c4, 0x4bf27807267a413e, 0x9cf0dd93e12fe1fd, 0x5864c256c1ff3184, 0x177871622f797b44, 0xa686429557fb1764, 0x6a37b37175f38380, 0x6a2369bc321d33db, 0x4c3a7376b305afcd, 0xc057ca4cd605bc90, 0xba07fea0185e44fc, 0xb2e63a18f012d997],
    // BlackPawn
    [0xe0d5ca916098a0c5, 0x76a33a6e33c8e83d, 0xa010cba8d1806c38, 0x5e200c9693d9ddbe, 0x6daaeeff40887207, 0x6ac01109927141b5, 0xf629f0d029ab8867, 0x930382c17ae025d0, 0x2bdda5aecc2e4130, 0x9c2cb6e4fa9fd4ee, 0xf54bcfd870690f70, 0x51cbd5a6e6690558, 0x74791ceee8ddf7a6, 0xf8bfb13a30b6f44f, 0x95e26a9236a617db, 0x794704aa36febb5b, 0x753a761f852badd4, 0x87e21e1a9209cfa0, 0xab8c4b82de778fd7, 0xfdd5d8cfacb72bec, 0x92417ed599bbb1ce, 0xf45d06ebb732db5c, 0xc1fab3660ec5d83, 0xe72d5f9ef9fdb57b, 0x28c23bfcc97f3870, 0x49a0e9eba1ab77d, 0x2e880f3f05760896, 0xbc0b7b5a9478fa12, 0x2cc53717da548a8c, 0xb75bd942f4ecdb4d, 0x657892ae725af31, 0x3ab72305dfc4a525, 0x401d785919c17024, 0xb6aff86a09e18011, 0x4ee19fd97c89560b, 0x7174c2aa44ad7428, 0x7e6d306177d5180f, 0xd306a71f98dfafd, 0x87c92dfb99d86f7f, 0xfe88e260afdd5bbe, 0x28e350471361cc87, 0x6961cbd161f1f910, 0x8d8642130587018b, 0x96f1309dc7a5dbf9, 0xd3779a7fccbbe226, 0xdb81cbf42d707bd4, 0xd35ca0d584cd311a, 0x6959db43bd510bf6, 0xcb638800e42db389, 0x515ea9913a5e7b72, 0xabc14a904ae08032, 0x581e0edf75922592, 0x9ee533468f997874, 0x5f6c3f7974edefb0, 0x156c8944cfa14d88, 0xa56e110873ca00b3, 0xbe40c3d0549bf09b, 0xb3f21a17b26a09a9, 0x1f255f2329edcc98, 0x2bb30a4d00a1eb56, 0x248e7052978bea0e, 0xa99c0e97708f6070, 0x14715fdead8b501b, 0xe8fbc234ccf397e6],
    // BlackKnight
    [0x9bb2fef594dfaae1, 0x9e2ee8a52c98e339, 0x3a00f98c3448dc52, 0xd3922d50121af56, 0xb7cdd40671bb90cf, 0x6aeaa0eb57f019b4, 0x78f3f3968888b98b, 0xad67fef318b3febe, 0xee5568962a660d1, 0x39c2ccc23adf6cf5, 0x51433dae28f25973, 0x9f3643ac3a4b8fbe, 0xf5affb1517c6b6ff, 0x54d5c817ced4435f, 0xbf6c512000ae5cd9, 0xcc9f1773255e10a3, 0x6acd9e9cb360cf0f, 0x516b524e3dd65881, 0x991a51f429e48f4, 0xcfe0e8bf292af220, 0xbda3bf32404ccc8c, 0x37713ab55093df63, 0x4d76198deea41afc, 0x22a49a6081229378, 0x91bbfae9183c4a1d, 0x5c6284710e68b39a, 0xd04fa9ce6518a69d, 0x5073d3a6e992b0b0, 0x350e8621c74fc95b, 0x4093b1901bc135a0, 0xcca4a5ed100a4897, 0x6bc501d03949a27b, 0x9447b99df425a889, 0x3a6513953c76cd6c, 0xa7674c077de3b94d, 0xb925238d1004e3e5, 0xbc304a46508be1ba, 0xc65058d2494caf27, 0x9df3ae9d1c559ebf, 0x175fd2c36cabd855, 0xf4e963368b0d323b, 0x9552e078a83e8d47, 0xb9544f3ad9688f7e, 0xcf8d464dfd124e93, 0xe6a3076a113fe673, 0x8918917072971b37, 0xa5e016a403d6d520, 0x1bd0ea115c205966, 0xc1ad93bfeadd6e91, 0xb9606f6c832f1741, 0xe3e0bb1c9bc78533, 0x738d98cf960d7002, 0xb21a8e92747f220c, 0x3de9e47fc54de02b, 0x42a701200afd90a2, 0xb21888cfd7c5ecb8, 0x937afef3cd518ac9, 0xada77ecf1a313dc6, 0xbb6fd1d4075122cb, 0xd5cbb57fe2ff8df0, 0xde05fbff9e96bda, 0xd8823283cdb7bec9, 0xefefd225dfe436b, 0xb95b4cab4775df40],
    // BlackBishop
    [0xe633845a513bc5df, 0xf737cb5f05e6c957, 0xb0994f0ae9ac2644, 0x13d6e372285d38b3, 0x8f52312e7bc1daab, 0xfaa6db9c4cb1600d, 0x67ee2df775b56c91, 0xf1833f82833047d5, 0xc554c4fe769966f, 0x36e3ad66d7bcae26, 0x7206a77ec3bd2817, 0xf7ce12d016f024b1, 0xa38f28aa2361c940, 0x89f18fdfc354eb98, 0xe438d966e0af50c, 0x7c77e5cfeb8f8344, 0xfb9a6a693c78af58, 0x9db7e77121578ff9, 0xf92460454f413f2e, 0x47c2dbfd3169b64c, 0xec96d5efec8e932f, 0x1088b9391a159974, 0x4ee7be5d3b310808, 0x520b9949d949fd89, 0x2d5de103b9e05280, 0xce756b774d293054, 0x2cc66bcad0a53ac0, 0x44f43c1a8e8fdd3e, 0x4086ab9c6617dc45, 0x6d6f1c57eb49715e, 0x9d9aea480b97c540, 0x9a538bd82b0f83bb, 0x6a0b531147898b48, 0xd17f912afd0baad0, 0x407ec598dd26ce7e, 0x6354681671a3bcfc, 0x8f484a4154585b74, 0xa27a86455e7a87c8, 0xe8a6151adbbf7b58, 0x9de751d08cc7a67e, 0x10873be7bed88512, 0x2b85427a5334262d, 0x3ff28c8186076c3e, 0xe26de3d80b10185f, 0x4404690879acb0a1, 0x1ee09982b28dfa55, 0x16b887dbd6ec781, 0xa6c2de338b7be3b4, 0x3a49fad353fa53d8, 0xefdfaf575747c5df, 0xe80fd7a16f63e243, 0xf672cf60b7da96bc, 0xe98c924eacf66ed0, 0xc8d08be4b09715a5, 0x8e6f80d528c75b0e, 0xf395af573e6b8e49, 0xa4247147f26f6058, 0x5633b1b52dd4ff80, 0x9b3416d8a3d95c87, 0x5a8b098054d392e4, 0x6f0c8790aed7c1ef, 0x9e3423bbbd0c6d7c, 0x74f77c158417694a, 0xf95906aca4dd9229],
    // BlackRook
    [0xf96edada7a549a4f, 0x9666804fcbae46e8, 0xcc529051771f9db0, 0x5cf0d660baf00ba, 0x14187f0452458656, 0xa620618dc4da6463, 0xbb1f7b7d9d854a6c, 0x365a69fa87a25b44, 0xbf799acc583b5654, 0x9c70fa585cbbe14e, 0x990481974e1bd1bc, 0xbb697d8979f47b11, 0x35f0ffaa3369955, 0x6a80f5f0ead570a5, 0xcc0f923d159cfc73, 0x1c4305f03894e43c, 0x6c964d99019d2d1c, 0x693237a9786c9952, 0x4cba8b91dcd16a93, 0x7128c56e4687d682, 0xe7ad07cd7b636501, 0xe1b9f920678f32e4, 0x2a5c00da73e4d368, 0xf8566c9a6df37df1, 0x96754356e3264b58, 0xca9cedf68753bdd5, 0xe5b0f869c139519e, 0xae788197057ff7af, 0x7e7b767befbe2b11, 0xdb5bca35b3c1f2ff, 0xaca6b4ef6d3bafd6, 0x3e1f5aafd086efda, 0xb1273cfd8ccfdba2, 0x7d2181eb2c99bcba, 0x2db1e2a92c9c0855, 0x6b0d2e771a8d0bef, 0x6068f2b1499cb9cd, 0xc66b0e3b337ecb27, 0x292744fdf6c29ea7, 0xb179d4a52d7c55d4, 0x8ffe6d95b2fb856a, 0x1ec85ecebde16183, 0x35076eca12d6d5dc, 0x5047853dd365bf11, 0x820a0344729444bc, 0x5a24837f0acc8c46, 0x713593b0b4ab02b7, 0xf318984164f543ad, 0x33ae426b1c4a6cdf, 0x74a5acc58042438d, 0xbbd6d82a75633564, 0x8e184aa68391fed2, 0x295c7ac902f3a942, 0x5e3adceb6b540b70, 0xaa3fbd00d56e6615, 0x3a5a6d9c5ab4a7ad, 0xe45035c3874ad01a, 0xfdc3aaf4072abaea, 0x54c21b420aed3bf0, 0x8d4ac9ea8eaa3f5, 0x38278762cf77d666, 0xaab037ae99849a05, 0x1e944f82aae5bfad, 0xe93dea1bb4a57d46],
    // BlackQueen
    [0xd8821160ba941eea, 0x320757d010f61d7d, 0x301774ee7850271d, 0xffcb7d706b95850e, 0x8f7ec46753ea1fac, 0x898f265b940fbb6f, 0x742fb6ecd2f567b6, 0x56690980b2340a24, 0x6db2acf85592ac58, 0xc3ed92e3bc5da23d, 0xce0a90b61a050b46, 0x44700625ab6209ba, 0x690ea5ac911ade5e, 0xe99d1fd98def1974, 0x70aa0dd5a07ab46, 0x5d7add3b8c78de44, 0x218333bffdc249cd, 0xd6860abb9b181bc2, 0x320697673191e2ab, 0x90eca488157aa927, 0xf77ab74466e8a0e1, 0x69e971ea0bac4688, 0xd569d134bcecf78e, 0x746add84b9bf26c6, 0x25b8781e3e175a70, 0xc698fd79fd26c355, 0x2cd38bf0c038c518, 0x2f7b726922a08bde, 0x89d01d0cc25e0afe, 0xc23da040007fd5cd, 0xdad8da90cb986588, 0xdd06cd31f4656c5a, 0x71bca07a6166078c, 0x59842ba49b8974d, 0x468b86e0f8376f9b, 0x5b9137425e67177e, 0x68ee457553ef7a6a, 0x5b2823ec8d02aa53, 0x569fbdf668f3f493, 0x1afb34f58bbbfb49, 0x643dff9efc99defe, 0xd2dae4adc353d5b8, 0x70decf8c505128b, 0xe5ffe2ac6337150b, 0x3c223eb3c9e4ea58, 0xc03d3c9f414517d6, 0x93ded8d0a3ecde52, 0x7dd22b21c5eaf806, 0x449a0a6c8f0f160a, 0xf40b299f55620af9, 0xde1ee60beeef5627, 0xf57b02386b651498, 0x32178351361c5d36, 0xd9d1c37eaf75464c, 0x195c750d775b5420, 0x5b5dc97e2d6e8b55, 0xf9976d7730c02af2, 0x46c319524fcd2d14, 0xd147bfb2d796b8c4, 0x2ac1c50f2bdc6645, 0xead1cfc56a053689, 0xd7116d279ab40344, 0xfe69ab968fcc9fc7, 0x7a14b2b448430be1],
    // BlackKing
    [0xf51133e94810bd7e, 0xe3a2702f6853229f, 0x9fed08fda3a2f0, 0xb6d1c473592d964c, 0x3de90749a7d4c425, 0x84868c35e096fbb8, 0xe8cce514acbbbc86, 0xf24f47e10036bd71, 0x57c2d8026702d2d5, 0x408de973fce5599c, 0x2b0af5de8dea8c6b, 0x879546729af05b0e, 0x80414b25ef50ca55, 0xdfa430c757ebc5c6, 0xbbb4a905125664a0, 0x57313847f6e911ce, 0x6d02e2bb78f3e7f6, 0xfcbf2fef4c48e5dc, 0x2cccf3ecd1d5aaef, 0x909eb7327c220826, 0xc4cb35d493ee1cf, 0xf1a236a39352f38d, 0xeaf287587d0a5352, 0xa3dabaa338a71dc, 0xa742518d8f8185d8, 0xfdce0f9ad85d7c34, 0xcd25bced8de3740f, 0x2f079e43ed5703c3, 0x3d2b3d67a24c7c64, 0x3594238e46eb9c0b, 0x787616f441874135, 0x4545eea66e3d6ebd, 0xa698d535d181adc4, 0x3ee5fa71c40c8c9e, 0xab05c876c3dd437d, 0xa412c907eaa4840a, 0x3e8e7219a5d639b9, 0x9395d88f581e4f73, 0x4c3ab86d728633ba, 0x798b334a5b6a248d, 0x8bca451624b71ac, 0x9f2947ed6d2c0ebd, 0x39965ce6a65a9953, 0x2c4b01ded8fad94f, 0x6f8937b74df763eb, 0xc8857f62f3eafe4d, 0xce839e7e0981d698, 0xa4f8cb93b5edc3eb, 0xe3480f9499bf12e7, 0x52e1b2e01e14e929, 0x327c39be29d15d8e, 0x63c4332e135fa762, 0xbbb1df76ae16bc2e, 0xc97bc270123d8e23, 0xb6fa0b10ba9cffde, 0x6762e6ae3943aa14, 0xacf30cf01bccdd61, 0xe40142a3325983c5, 0x8c5b3f025fa81cb6, 0x832e73d2fc527423, 0x662a9dbe25e35567, 0xaf4a41a7add2f7f3, 0x930cd4a3bf7e34dd, 0x3173b31494dd9237],
    // Reserved (Black nothing) - not used
    [0x8597fd0b53200036, 0x49df776cfdb2b653, 0x2b381efdd9db9801, 0x656d79bd27677766, 0xc926852edc481be, 0x946e2d6acfcd4262, 0x81b5de2af64503ac, 0xac1583dc7f91ece6, 0x8ffe5656540eaea8, 0xef55b9c73c9ca6f5, 0xf5daaccf64a860bc, 0xfe17675e01f4003, 0x172773e26bfd6f33, 0x43fa3a7ab3505163, 0x1ada68ed66ef8581, 0x6e4f8a33a8c9688d, 0x15c0da35831b6f87, 0x2ec9ea5fbe817bea, 0x9c0b3bf5fcc9e98, 0xb958026689b2d4e8, 0x91764d319c632186, 0x2072ccbe2db643b7, 0xea1181341d52bf51, 0xa7472cf0c5f53242, 0x7d29eac7bc9dea08, 0x65b88b50632b3a1a, 0x8ecda0b5d1e90a58, 0x9680050c4909b50e, 0xba4aea3ce7d24327, 0xf86b8e3ec5272390, 0x18e8bde6b2cd019b, 0xfa219f70df1fc242, 0xde7c95cd145b2c2e, 0x307616d4fd9a8b13, 0x8b5539d191e4e0d, 0x47784a907e104337, 0x7fa5157435803f70, 0x8d08e91d5a9e7666, 0x5424e7d43ef741c9, 0x259d8d052c53f708, 0x910f9009fe78d940, 0x9ecfb2d99462f0f9, 0xdbbb8065a6e3dd28, 0xd5141e188231959e, 0xb7de46be9f6f12de, 0x5c76a5040532583, 0x9e23f0219893166f, 0x11050a8d194e25c3, 0x93e481e4c38edecc, 0xbc41e7743d9fd5f3, 0x270f54923d9a4772, 0x8350ed06876d61bc, 0xe2548d1ac089e2cf, 0x242e08fe337d4698, 0x7b7fb1e8bdf99838, 0xb259e9dfbce6de82, 0xcb13b49dacfccbce, 0xe236d2e4a55a5b7f, 0x6ffd8ef78197f16, 0x8a973f03992ede0, 0x3673c700ef42120e, 0xb1937c70a7a97e90, 0x8ad9c9bfd0675ae7, 0xcb71e81c163a6e46],
    // Reserved (White PieceNone) - not used
    [0xec013cad70970df7, 0x84cf2bbde8637b70, 0x556cac9253fce039, 0x972349e54261288e, 0x1cb4c0e7cb77b341, 0x4852ca437ebd3742, 0xa042ac1f05251aa0, 0x11dbaba0d0956b57, 0xea0aec958e6c6096, 0x909bed1a781b7426, 0x4742ff36845663cb, 0xdaa8aab0ba374685, 0x659a42a9b54a722a, 0xeb8b1798bdf4df48, 0x3b28fd18566aa6d6, 0x979829a6b7187596, 0x75ee2675ab827b01, 0x4a08f1244fdb6141, 0x6f9b32fa6cf597b, 0xaa7f39e3750acecf, 0x3708aafda0bdef0, 0x7d072aea96a8fd69, 0xa31b8b8c3a0e2f97, 0x5596efcb2cdd049b, 0xf122334ecb4e2663, 0xb00977343799897b, 0x9838e8904dd2e123, 0xaa6a5c09fffc2ab0, 0xe15e0be33cbfad13, 0x9a0bd674445faeb0, 0x4e7da09404f8962a, 0x4d47283bd6abb783, 0xf895f3181f818b2, 0x435751054e922cc1, 0x2267d450e0902741, 0xda299bc917920439, 0x4958b383600ff5ef, 0xc94eeca1cc7dbde, 0x6672b3a100f21a9a, 0x3afd147e55b6eb63, 0x1ee8ac1a63ce4053, 0xba366b662a7ca018, 0xb6a5ba2ea2f52a67, 0xcf70efa171a95d21, 0x131964c6a9ad0911, 0x955ced5aed79d726, 0x6183abc03fd9fd3f, 0xd1daa30b1ca6da46, 0x6ea1a12496b0dcd1, 0x8cee057b797bb7fc, 0xf4468d1eaa8a527a, 0x9e5989b80e2f7c20, 0x94a0fee5d6fdbd7, 0x2989dc2e9dcecd30, 0x95d74e0e9370c8c7, 0x24fe380e7d3d9d84, 0x40d8371e484c42c9, 0x8337cc645d3583cb, 0xc1a377e37c5786cf, 0xc01961ad9c9f60fb, 0x1b21cb42fb68eb38, 0x5efa83cd88f13fad, 0xe3a36666bc5e85ba, 0x9540b070bb3e4912],
    // WhitePawn
    [0x578b2a5016f37702, 0x91435781d8f7b5bc, 0x4d46cfa91839e285, 0x1bec1e7f8ad23349, 0xf87107123b56d890, 0xe55a6501eca1469, 0x38b19a5e496f299b, 0xd0d1b126e0ff2fa9, 0x433c7f13d5c3dbd2, 0x76e6083833511417, 0x9dfae4481556127b, 0x2cbe2af363d856ab, 0xb9091e726ea68875, 0x450736b6618a01d7, 0x4a2f84c6d5996d8b, 0xa540e0c431f66731, 0xee0a72402fa89c0a, 0x2bd2cc381ebcc278, 0xabb8d92304ebe76b, 0x68ab34b9908731bf, 0x27c544252a4c7f38, 0xed8f9e3bc7db7577, 0x99aba6e053eea9d9, 0xbdd1622170d6548d, 0xee09ec3b9f3bc31, 0xc3dc55df90b357a2, 0x1e7460e1134718d1, 0x5e20fa9a086111ad, 0xfc57cf963c6d22b5, 0xde41128ad862f9fa, 0x616e638d36e1f9ec, 0xbd758f38c491e489, 0x757fe3574f34d667, 0xf056e9be42f2c0e0, 0x8058b306cf40632d, 0x521aa0fe961e09cd, 0xf9268cc7bcea1b32, 0x64fc94729b61ced5, 0x99368644dae0dfb, 0x2e89da0ea6d63b4b, 0x56d4b151e8e37511, 0x1e17fd4057f51af0, 0x9cf4f98056851e01, 0xbe94ab1a73a1d149, 0x1182730d837dc42c, 0xa30155fddacea982, 0x89c8d65e9f6ccb1d, 0x900d3fc771f5a4ec, 0x81e74971c9f7a535, 0x6da6f84d8cdd359b, 0x4d14235cc35e991, 0x9555c10e1d788d64, 0x5f2fafac7f6df58a, 0x40619f0f55239d6e, 0x13890e370000de9e, 0x9722e6adba43be9a, 0xeef3a5acc2edf834, 0xb2a6ff040d54a755, 0x4a8082f38673b365, 0xfb21ef13aeab18a, 0xf4dc7f5f58531e0, 0x8588519d3f043207, 0xd74f76781cf80fde, 0x871d2802808a1d08],
    // WhiteKnight
    [0x5e54772cd1a61057, 0x7ea0f40b10b5a430, 0xa85076f610c3d44f, 0x2f250bf7e859328a, 0x210e61d0cf6949e6, 0xe6a6e36f51b77d3d, 0x251f2b3a3362c5d9, 0x6c769d9169285392, 0x138600249d43bde0, 0x1010f808a6ef9f11, 0x18b653e7058c6add, 0x990e5a7af86dc61e, 0xaa84ffda0f42d5ce, 0xdcd6bd104c218ca6, 0x83a34daf694c8d8e, 0x7a84a6b02ab20822, 0x419207277864dd8, 0x9cc9d4b27f55114c, 0xe2b7b97c9554a2a9, 0xfa087aca33514917, 0x532699220903ad7, 0xd7a758c916e734dd, 0xa228e0db787cfa34, 0xfe50e596ead50e85, 0x8c906bfe825ba26e, 0xda29e03389c37f6c, 0x950a06bf9a46dbe9, 0x32e3cc26344eaeb9, 0xe08e2e21ba36f156, 0xae0fd85349c6dd1d, 0x269fa3fe536c6dc, 0xe2c18f3185b7269d, 0xf9de95260a5d6f6e, 0x5e6e797c9b19389a, 0xd3ef160a962121c6, 0xae96931df7e0c416, 0xaf4d967d69301d76, 0x8a355acc4a9422b5, 0x5a59a3139dc137a7, 0xf75605404646f1f4, 0xa5814c074aca82c5, 0xa67ff6ec0badc82f, 0xc085c9890632f646, 0xc7dfead14566c59b, 0x2443fa1346e027ea, 0x41af4ae4f73a82e4, 0xe68f3466ce1f7040, 0xf863226cdd185d97, 0xefaa0e52567ea45b, 0xa83a3f4bb6c16702, 0x76fd0bf20fe1e299, 0x455a87f794f55242, 0x5f764efd0e4170e7, 0x888425a8de26dd9c, 0x48b8a167932bdd95, 0x85bb002dedea4d50, 0x1cf88ddb53ca40ce, 0x6304a73be848373b, 0x52c78b7677165fea, 0xd354a8821bf6e474, 0xb533185cb6e5bd17, 0xc4274897ff73d4d8, 0xf4b8b539cd19cf40, 0x2deb81bccfb42eb3],
    // WhiteBishop
    [0x31abc05e941633cc, 0xffc404a44dca4c9, 0x518c0fec16de9d5a, 0x4e9525db04cde31e, 0x4a2bed0199ffafac, 0xb8fe19c46486940d, 0x9dbf468d00675fe1, 0x12b2c3d6955970df, 0x3e5cc9a3134b0f2d, 0xb433852e04a23b2e, 0xd06c36212bf43cfc, 0x133e1171e3313480, 0x4e8c38ec94781d41, 0x7933965d95272d4e, 0xbac13165e90626f2, 0x45a35751bb916f7e, 0xecc53643ad35af37, 0xd797272b756950eb, 0x6280b457807d642a, 0xe5bba2d4858353d8, 0x4f56bcbb660eb085, 0x606240f6fdf266ae, 0x4ab91958428d11bd, 0xb40867ed3dceaea5, 0xe5de0433a706a46b, 0x3b07e7f66e31fc5c, 0xed5ea0943181867e, 0x922a83cb5d120437, 0x73fb7c5c71751884, 0xe2d7548a7ae49425, 0xbdaabe6edf693d22, 0x47cb357450b87d88, 0x82b088390c0b10aa, 0xfe49e9dbbe94d70e, 0x1f68def7b63d0757, 0x79e8147f0d72abdd, 0x951bbe1e7e483cf4, 0x2918900d5b80c693, 0x70af725ecd242088, 0x9447a866b0faa9e, 0xc4ac12565026ebd4, 0xdc1cdd4e07aa1734, 0x41fa6f7311b89ed, 0x46f174f6f4a2801b, 0x8b68ac118ee0e324, 0x2091db04e0f26976, 0x11f8a0ab9c964bb6, 0xb15fda9571b9bf23, 0x358162b85760e627, 0xc860e1709940a825, 0x6e3e6f335a5f2147, 0xe1f170654f24d6c1, 0xc3db478a2f0269c2, 0x5d554f1370875618, 0x5ed8b04508c4c2f9, 0x4344db6347602dd, 0x278089d22f6a5a5, 0x51b6c0d428e6fd96, 0x596d95244ca19ddc, 0x7b2b907f6e37b2f4, 0xd8974a720aade6f0, 0xa66a0894ed40e10, 0xea1038bea825dafa, 0xebf05aa0c58f700d],
    // WhiteRook
    [0xe3dfcbe606b1f205, 0xe03886e465811c8a, 0x20b88fc979c12de2, 0xc2bcf8cb31f64297, 0x4490443ddcdfbf01, 0x38130cd1cd7bd8ca, 0xa35ac6a8703ad432, 0x6ebbaac0991b64ca, 0xe579dcc9d3ae7fe0, 0x7a1e0cee137df83d, 0x9ff1feab0f0586a3, 0x8287838764c73304, 0x6aabcbdde59e7772, 0xe7afb93e49e8501a, 0x95ef73f26d0f66b9, 0x610d1f6a44ae570c, 0xc317447eb58a4ac3, 0x379d4dcef30c58f4, 0x7b318a67872f61d1, 0x311732f990b73abc, 0x88a89fb1d7e84ea, 0x6d52695e266f2682, 0xa0dfe41f16d24085, 0x37f00fa839467fcd, 0xde7445937765bc27, 0x8cd74d559dd8849a, 0x3a0b4d922ef9cd39, 0xb950f7e812164e1f, 0xd59f070756273854, 0x73adf8790c6c345, 0xd5d1a6459ed9bf0e, 0x31abd573257d6b4c, 0x529b7716a8230056, 0x41e7dbb98b2f6a22, 0x104eb66a18557027, 0x2b20ed6fa186794b, 0xb36eeb33e597eac3, 0x3a37e1a2ca3c3ba3, 0xf88fde47db3b915c, 0xadf5f1bb8918d2db, 0x522917ad23c9c5e3, 0xe5b97aaa589abf1c, 0x9aa7d8232cc93dfd, 0x2851d4f400ba8956, 0x1d1014f5f495edaa, 0x831d5a3a42626b6a, 0xad6100234118ed36, 0x4ee2fee9401786c0, 0xf599a4a9d24eb7ef, 0x80d2250cb395999e, 0xf1cc04693830238b, 0x44121af654d4c163, 0x55efde0325ef61c8, 0x5516ce64560cfedb, 0xc4294c2100c7e5c7, 0xafffdd5b2745c211, 0x55d7afb1986fd5b7, 0x7e3246b2fefc0bec, 0x304a2d1a95b7e1b2, 0x744afe6865dc145b, 0x9ebbc0a4eac60747, 0x2975ea05d3a03f5b, 0x2baf99c50b285538, 0xa7b3cfa1a4667728],
    // WhiteQueen
    [0x34ab6c46669522c3, 0xf3fe7849780bf29d, 0x97b3fb8cc3d06ec9, 0xa76bc68a0cd00157, 0x1a81704d9b5dfcf0, 0x1e15bdbb1345bc92, 0x38db643d6f5762fa, 0x9d76cd2b7d893a59, 0xba17f1f9e2d3379e, 0x4722b7c919be4bfd, 0x9cfb1efdf3d2d18a, 0xacdbe9d03b146f83, 0x9a382c3bd8b39197, 0xff2354d3a6c51318, 0x8efc01a9f60a3c0c, 0xc9801407810085d2, 0x58768bfd161b89b5, 0xb7f15406c1e4e94e, 0x3f70a7ff579a0533, 0x6fbe3eaa038d2653, 0x222fab2066d67427, 0xa8c5117b5ff8162d, 0x1bcbcec3020e7db1, 0x811cebb660636753, 0x29d99ef64cb4c257, 0xc526c4d6d8636bb6, 0x2b676e68736dc393, 0xd50f352f3de9fb5c, 0x650de4f27de735d0, 0x6de43336d185eb27, 0x2a20d24e52e2d5ec, 0x2b9f5309d61abfb2, 0xe1acdae33f72e62b, 0x230ab46beefcaa07, 0xcde2c6cebd69f2b, 0x1405f53cc4b04429, 0xd6114161554fc71f, 0x1819d0fd7207e9c7, 0x8585837e0a7bb92c, 0xa25c3c223c5b4d3, 0x4edff02172cf01a9, 0xf203c914e22d3ac3, 0xd8e954e8685821b7, 0xfe4d2e826ee6413c, 0xa9d8733932dca8c4, 0xd82bd393bb2f67ff, 0xd36633a483c46828, 0xe4e26d1077d46f90, 0xb37bed1c634ad2d, 0xd0bc3a7d92329192, 0x9caaff80ff5c58d2, 0x873d5cd0efb6d3c9, 0x1c33599c1054d993, 0x8dc3cb32de9923a9, 0x876553d6efeeeba5, 0x7e4f1a0c9b4a096c, 0xc1e43345874cdf35, 0xc812e6c4e0967c1a, 0xa25d113521c94715, 0xa9d8d33474763618, 0xcbea58460e1625a0, 0x491e46a27f402074, 0x8d1cbad61e7308b7, 0x438777b61254dc12],
    // WhiteKing
    [0x24c96aaf489c0a1e, 0xa7080635dc7d3e25, 0x725cd6acb4a82ba0, 0xa2c72f2d7fd60684, 0x1cf766c925ed6c06, 0x8872d8a4cc7acd, 0xbf8dd26317317014, 0xb690c804c71d1985, 0x2dbeacf57784e401, 0xfaf844272d1ed26a, 0xcf4aebef369e1fc7, 0x398b31fcfae5da3b, 0x2e8867586329126e, 0x4e61ff80ab9ecd47, 0xff97d2af7e6e0f99, 0x44d23efd5acc37ec, 0x446fe0a426abc3cd, 0x7b05dd4357769c59, 0xe2745779f9e60f74, 0xa99504d15b058bcb, 0x975000993f0ba81e, 0x80e32e0aa9a9512e, 0xd692fda81a735b60, 0x99e8e1f6445fca23, 0x8fe0fbf102b1733b, 0x9d7b4a68e0c552b3, 0x7a2cba8d3ccfcab4, 0xe67764d01fcd011f, 0x600598fefb4e54c4, 0xe3482e248c60969d, 0x88f4edadae23c078, 0x49f5394174ccacd2, 0x1b4002c0f8bbe0c4, 0x2866645f97225ee6, 0xe82a6fe0a3e13252, 0x880fc97b51eedb3a, 0x14025f382a50a605, 0x68904f1223c11dd7, 0x302aa562dc2643bb, 0x6edb4da188c4e470, 0x55304e74cfbbf6d, 0x591149e50b18aa02, 0xe4633a0e478c9e91, 0x6f72b41bb03626a6, 0x4275b155c63ca092, 0x53cc274ee7b471e9, 0x83e29bdcc274d41a, 0x4998d2440577f90b, 0x19d5ada8667dbcb4, 0x33df0e1de9c89754, 0x93695e862eb389fb, 0xc1e30a7476103f12, 0x6e1c6cdf4d1da2f2, 0x93062f2bc0dd1bc1, 0x8573cfa6d92fd486, 0x3c362dad7c7a386e, 0x82ba2297edc9370, 0x7d76eaf991eb153, 0xb7c42518f5e30a8a, 0x4d26db14b344a7f3, 0xe6d0979945507b4e, 0x97b2ec348a5cb66, 0xe32cc5b48ec22b78, 0x45f81537629cd40d],
    // Reserved (White nothing) - not used
    [0x387b228bb078c476, 0x143f709726369bf, 0x454bc11d8a89b967, 0x22aef8dffe3ab844, 0x33876a35f9788d06, 0x8c38d96c50d860c0, 0x4b15a087a2308fbe, 0x198ca07a1964c5d8, 0xa802349e0cbd48b2, 0xfde744ca29115313, 0x3bdaef9a80d3bd2c, 0xcc5c5a5df8d79140, 0xee11d6f92cfce7cc, 0x7b2dee41eb8b9b48, 0xad06995c0454ec7c, 0x20ccac8232e7a37e, 0xfa3c3eae3134db14, 0xed4183fef120c45, 0x29d9a416536a3ef4, 0xda02622ac5d2027e, 0xa3c801a9a628843d, 0x1f48dd48df03be21, 0x4516b67b94ab79aa, 0x2d7cebfa7391e518, 0x79305e3074006c19, 0x37bd6f1e479b4b6f, 0xbaccaece6d959768, 0x556c2087d106dc39, 0x1b9d7f1137483999, 0xb788949f8cf8d320, 0x8ffdf77a648892f8, 0x815c6a651164d94b, 0xf4d8e34d50077f17, 0x1ff23c80ac73f107, 0xbbf1b301a4bcb0d9, 0xfb6773649924cc79, 0x44350a07dfb4619, 0xc49235de34662571, 0x8dc26c0e33bcd49f, 0xfcacb3a1ba4c6758, 0x96bc2841ab02edb6, 0x1f3bc70aecf53a2e, 0x1d404a306bcadfb5, 0xa91d0a3689738bf2, 0x8e5c6abfa1a16a5, 0x7010dc957d6f9618, 0x4bf20186d38c62fe, 0x7496099910bef2fe, 0xa1c49d195d5b2ddb, 0x24bd97c18a39bff3, 0xe45d2810e8246d11, 0x40cc28cc35467dbb, 0x1d11408ef80a55db, 0xe3b789b6a5feda54, 0x75cd73bcfb0ee8df, 0x3eb495127e8510d8, 0x7e4be3ee3d733018, 0x33e22c4dc1d996cd, 0xb5b6f3e5c386ad5c, 0xda37c1ebd7e7be0c, 0x1902a8a71f614ede, 0x5352d73832e945c7, 0xa4a73a1b2885c24d, 0xf44148aa03dde06d],
];

#[derive(Clone)]
struct Bucket {
    full_key: ZorbistKey,
    depth: Depth,

    nodes: usize, // TODO: custom buckets

    #[cfg(feature = "transposition_table_checks")]
    fen: String,
}

impl Bucket {
    const fn empty() -> Self {
        Self {
            full_key: ZorbistKey(0),
            depth: 0,

            nodes: 0,

            #[cfg(feature = "transposition_table_checks")]
            fen: String::new(),
        }
    }
}

// NOTE: instead of using Optional<Bucket>
//       we interpret buckets with full_key == 0
//       as empty, saving 1 bit (and 64 bits aligned)
pub struct TranspositionTable<const SIZE: usize> {
    buckets: [Bucket; SIZE],

    // TODO: Collision stats
}

impl<const SIZE: usize> TranspositionTable<SIZE> {
    #[inline(always)]
    pub const fn new() -> Self {
        always!(SIZE <= 1024);
        always!(SIZE & (SIZE - 1) == 0);

        const EMPTY: Bucket = Bucket::empty();

        Self {
            buckets: [EMPTY; SIZE],
        }
    }

    #[inline(always)]
    pub fn new_box() -> Box<Self> {
        always!(SIZE & (SIZE - 1) == 0);

        let mut result: Box<Self> = unsafe { undefined_box() };

        for i in 0..SIZE {
            result.buckets[i] = Bucket::empty();
        }

        result
    }

    #[inline(always)]
    pub fn add(&mut self, board: &Board, depth: Depth, nodes: usize) {
        let full_key = board.hash();
        let small_key = full_key.index::<SIZE>();

        // TODO: log overwrite collision
        #[cfg(feature = "transposition_table_checks")]
        {
            // TODO: FenBuffer
            let mut fen_buffer = StaticBuffer::<u8, 90>::new();
            board.fen(&mut fen_buffer);
            let fen = String::from(
                std::str::from_utf8(fen_buffer.as_slice()).unwrap()
            );
            self.buckets[small_key].fen = fen;
        }
        
        self.buckets[small_key].full_key = full_key;
        self.buckets[small_key].depth = depth;
        self.buckets[small_key].nodes = nodes;
    }

    // TODO: return bucket
    #[inline(always)]
    pub fn get(&self, board: &Board, depth: Depth) -> Option<usize> {
        let full_key = board.hash();
        let small_key = full_key.index::<SIZE>();

        let bucket = &self.buckets[small_key];
        if bucket.full_key != full_key {
            // TODO: log first-level collision
            return None;
        }

        if bucket.depth != depth {
            // TODO: log
            return None;
        }

        // TODO: check for fen, log second-level collision
        #[cfg(feature = "transposition_table_checks")]
        {
            let mut fen_buffer = StaticBuffer::<u8, 90>::new();
            board.fen(&mut fen_buffer);
            let fen = String::from(
                std::str::from_utf8(fen_buffer.as_slice()).unwrap()
            );

            let cut_fen = fen.split(" ").take(4).collect::<Vec<_>>().join(" ");
            let cut_buk_fen = bucket.fen.split(" ").take(4).collect::<Vec<_>>().join(" ");

            if cut_fen != cut_buk_fen {
                return None;
            }
        }

        Some(bucket.nodes)
    }

    #[inline(always)]
    pub fn clean(&mut self) {
        for i in 0..SIZE {
            self.buckets[i] = Bucket::empty();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut tt = TranspositionTable::<1024>::new();
        let board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(tt.get(&board, 3), None);

        tt.add(&board, 3, 10);
        assert_eq!(tt.get(&board, 3), Some(10));
        assert_eq!(tt.get(&board, 4), None);
    }

    #[test]
    fn remove_piece_affects_hash() {
        let mut board = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let hash_1 = board.hash();

        board.remove_piece(a1);
        let hash_2 = board.hash();
        assert_ne!(hash_1, hash_2);
    }

    #[test]
    fn side_to_move_affects_hash() {
        let board_1 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let board_2 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");

        assert_ne!(board_1.hash(), board_2.hash());
    }

    #[test]
    fn en_passant_file_affects_hash() {
        let board_1 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1");
        let board_2 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1");

        assert_ne!(board_1.hash(), board_2.hash());
    }

    #[test]
    fn castling_rights_affects_hash() {
        let board_1 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let board_2 = Board::from_fen(b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1");

        assert_ne!(board_1.hash(), board_2.hash());
    }

    #[test]
    fn regression_1() {
        let board_1 = Board::from_fen(b"7r/2p5/8/KP1p4/5p1k/8/3RP1P1/8 w - - 0 1");
        let board_2 = Board::from_fen(b"7r/2p5/3p4/KP6/5p1k/8/3RP1P1/8 w - - 4 1");

        assert_ne!(board_1.hash(), board_2.hash());
    }
}