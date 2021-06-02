const TE0: [u32; 256] = [
    0xc66363a5, 0xf87c7c84, 0xee777799, 0xf67b7b8d, 0xfff2f20d, 0xd66b6bbd, 0xde6f6fb1, 0x91c5c554,
    0x60303050, 0x02010103, 0xce6767a9, 0x562b2b7d, 0xe7fefe19, 0xb5d7d762, 0x4dababe6, 0xec76769a,
    0x8fcaca45, 0x1f82829d, 0x89c9c940, 0xfa7d7d87, 0xeffafa15, 0xb25959eb, 0x8e4747c9, 0xfbf0f00b,
    0x41adadec, 0xb3d4d467, 0x5fa2a2fd, 0x45afafea, 0x239c9cbf, 0x53a4a4f7, 0xe4727296, 0x9bc0c05b,
    0x75b7b7c2, 0xe1fdfd1c, 0x3d9393ae, 0x4c26266a, 0x6c36365a, 0x7e3f3f41, 0xf5f7f702, 0x83cccc4f,
    0x6834345c, 0x51a5a5f4, 0xd1e5e534, 0xf9f1f108, 0xe2717193, 0xabd8d873, 0x62313153, 0x2a15153f,
    0x0804040c, 0x95c7c752, 0x46232365, 0x9dc3c35e, 0x30181828, 0x379696a1, 0x0a05050f, 0x2f9a9ab5,
    0x0e070709, 0x24121236, 0x1b80809b, 0xdfe2e23d, 0xcdebeb26, 0x4e272769, 0x7fb2b2cd, 0xea75759f,
    0x1209091b, 0x1d83839e, 0x582c2c74, 0x341a1a2e, 0x361b1b2d, 0xdc6e6eb2, 0xb45a5aee, 0x5ba0a0fb,
    0xa45252f6, 0x763b3b4d, 0xb7d6d661, 0x7db3b3ce, 0x5229297b, 0xdde3e33e, 0x5e2f2f71, 0x13848497,
    0xa65353f5, 0xb9d1d168, 0x00000000, 0xc1eded2c, 0x40202060, 0xe3fcfc1f, 0x79b1b1c8, 0xb65b5bed,
    0xd46a6abe, 0x8dcbcb46, 0x67bebed9, 0x7239394b, 0x944a4ade, 0x984c4cd4, 0xb05858e8, 0x85cfcf4a,
    0xbbd0d06b, 0xc5efef2a, 0x4faaaae5, 0xedfbfb16, 0x864343c5, 0x9a4d4dd7, 0x66333355, 0x11858594,
    0x8a4545cf, 0xe9f9f910, 0x04020206, 0xfe7f7f81, 0xa05050f0, 0x783c3c44, 0x259f9fba, 0x4ba8a8e3,
    0xa25151f3, 0x5da3a3fe, 0x804040c0, 0x058f8f8a, 0x3f9292ad, 0x219d9dbc, 0x70383848, 0xf1f5f504,
    0x63bcbcdf, 0x77b6b6c1, 0xafdada75, 0x42212163, 0x20101030, 0xe5ffff1a, 0xfdf3f30e, 0xbfd2d26d,
    0x81cdcd4c, 0x180c0c14, 0x26131335, 0xc3ecec2f, 0xbe5f5fe1, 0x359797a2, 0x884444cc, 0x2e171739,
    0x93c4c457, 0x55a7a7f2, 0xfc7e7e82, 0x7a3d3d47, 0xc86464ac, 0xba5d5de7, 0x3219192b, 0xe6737395,
    0xc06060a0, 0x19818198, 0x9e4f4fd1, 0xa3dcdc7f, 0x44222266, 0x542a2a7e, 0x3b9090ab, 0x0b888883,
    0x8c4646ca, 0xc7eeee29, 0x6bb8b8d3, 0x2814143c, 0xa7dede79, 0xbc5e5ee2, 0x160b0b1d, 0xaddbdb76,
    0xdbe0e03b, 0x64323256, 0x743a3a4e, 0x140a0a1e, 0x924949db, 0x0c06060a, 0x4824246c, 0xb85c5ce4,
    0x9fc2c25d, 0xbdd3d36e, 0x43acacef, 0xc46262a6, 0x399191a8, 0x319595a4, 0xd3e4e437, 0xf279798b,
    0xd5e7e732, 0x8bc8c843, 0x6e373759, 0xda6d6db7, 0x018d8d8c, 0xb1d5d564, 0x9c4e4ed2, 0x49a9a9e0,
    0xd86c6cb4, 0xac5656fa, 0xf3f4f407, 0xcfeaea25, 0xca6565af, 0xf47a7a8e, 0x47aeaee9, 0x10080818,
    0x6fbabad5, 0xf0787888, 0x4a25256f, 0x5c2e2e72, 0x381c1c24, 0x57a6a6f1, 0x73b4b4c7, 0x97c6c651,
    0xcbe8e823, 0xa1dddd7c, 0xe874749c, 0x3e1f1f21, 0x964b4bdd, 0x61bdbddc, 0x0d8b8b86, 0x0f8a8a85,
    0xe0707090, 0x7c3e3e42, 0x71b5b5c4, 0xcc6666aa, 0x904848d8, 0x06030305, 0xf7f6f601, 0x1c0e0e12,
    0xc26161a3, 0x6a35355f, 0xae5757f9, 0x69b9b9d0, 0x17868691, 0x99c1c158, 0x3a1d1d27, 0x279e9eb9,
    0xd9e1e138, 0xebf8f813, 0x2b9898b3, 0x22111133, 0xd26969bb, 0xa9d9d970, 0x078e8e89, 0x339494a7,
    0x2d9b9bb6, 0x3c1e1e22, 0x15878792, 0xc9e9e920, 0x87cece49, 0xaa5555ff, 0x50282878, 0xa5dfdf7a,
    0x038c8c8f, 0x59a1a1f8, 0x09898980, 0x1a0d0d17, 0x65bfbfda, 0xd7e6e631, 0x844242c6, 0xd06868b8,
    0x824141c3, 0x299999b0, 0x5a2d2d77, 0x1e0f0f11, 0x7bb0b0cb, 0xa85454fc, 0x6dbbbbd6, 0x2c16163a,
];

const TE1: [u32; 256] = [
    0xa5c66363, 0x84f87c7c, 0x99ee7777, 0x8df67b7b, 0x0dfff2f2, 0xbdd66b6b, 0xb1de6f6f, 0x5491c5c5,
    0x50603030, 0x03020101, 0xa9ce6767, 0x7d562b2b, 0x19e7fefe, 0x62b5d7d7, 0xe64dabab, 0x9aec7676,
    0x458fcaca, 0x9d1f8282, 0x4089c9c9, 0x87fa7d7d, 0x15effafa, 0xebb25959, 0xc98e4747, 0x0bfbf0f0,
    0xec41adad, 0x67b3d4d4, 0xfd5fa2a2, 0xea45afaf, 0xbf239c9c, 0xf753a4a4, 0x96e47272, 0x5b9bc0c0,
    0xc275b7b7, 0x1ce1fdfd, 0xae3d9393, 0x6a4c2626, 0x5a6c3636, 0x417e3f3f, 0x02f5f7f7, 0x4f83cccc,
    0x5c683434, 0xf451a5a5, 0x34d1e5e5, 0x08f9f1f1, 0x93e27171, 0x73abd8d8, 0x53623131, 0x3f2a1515,
    0x0c080404, 0x5295c7c7, 0x65462323, 0x5e9dc3c3, 0x28301818, 0xa1379696, 0x0f0a0505, 0xb52f9a9a,
    0x090e0707, 0x36241212, 0x9b1b8080, 0x3ddfe2e2, 0x26cdebeb, 0x694e2727, 0xcd7fb2b2, 0x9fea7575,
    0x1b120909, 0x9e1d8383, 0x74582c2c, 0x2e341a1a, 0x2d361b1b, 0xb2dc6e6e, 0xeeb45a5a, 0xfb5ba0a0,
    0xf6a45252, 0x4d763b3b, 0x61b7d6d6, 0xce7db3b3, 0x7b522929, 0x3edde3e3, 0x715e2f2f, 0x97138484,
    0xf5a65353, 0x68b9d1d1, 0x00000000, 0x2cc1eded, 0x60402020, 0x1fe3fcfc, 0xc879b1b1, 0xedb65b5b,
    0xbed46a6a, 0x468dcbcb, 0xd967bebe, 0x4b723939, 0xde944a4a, 0xd4984c4c, 0xe8b05858, 0x4a85cfcf,
    0x6bbbd0d0, 0x2ac5efef, 0xe54faaaa, 0x16edfbfb, 0xc5864343, 0xd79a4d4d, 0x55663333, 0x94118585,
    0xcf8a4545, 0x10e9f9f9, 0x06040202, 0x81fe7f7f, 0xf0a05050, 0x44783c3c, 0xba259f9f, 0xe34ba8a8,
    0xf3a25151, 0xfe5da3a3, 0xc0804040, 0x8a058f8f, 0xad3f9292, 0xbc219d9d, 0x48703838, 0x04f1f5f5,
    0xdf63bcbc, 0xc177b6b6, 0x75afdada, 0x63422121, 0x30201010, 0x1ae5ffff, 0x0efdf3f3, 0x6dbfd2d2,
    0x4c81cdcd, 0x14180c0c, 0x35261313, 0x2fc3ecec, 0xe1be5f5f, 0xa2359797, 0xcc884444, 0x392e1717,
    0x5793c4c4, 0xf255a7a7, 0x82fc7e7e, 0x477a3d3d, 0xacc86464, 0xe7ba5d5d, 0x2b321919, 0x95e67373,
    0xa0c06060, 0x98198181, 0xd19e4f4f, 0x7fa3dcdc, 0x66442222, 0x7e542a2a, 0xab3b9090, 0x830b8888,
    0xca8c4646, 0x29c7eeee, 0xd36bb8b8, 0x3c281414, 0x79a7dede, 0xe2bc5e5e, 0x1d160b0b, 0x76addbdb,
    0x3bdbe0e0, 0x56643232, 0x4e743a3a, 0x1e140a0a, 0xdb924949, 0x0a0c0606, 0x6c482424, 0xe4b85c5c,
    0x5d9fc2c2, 0x6ebdd3d3, 0xef43acac, 0xa6c46262, 0xa8399191, 0xa4319595, 0x37d3e4e4, 0x8bf27979,
    0x32d5e7e7, 0x438bc8c8, 0x596e3737, 0xb7da6d6d, 0x8c018d8d, 0x64b1d5d5, 0xd29c4e4e, 0xe049a9a9,
    0xb4d86c6c, 0xfaac5656, 0x07f3f4f4, 0x25cfeaea, 0xafca6565, 0x8ef47a7a, 0xe947aeae, 0x18100808,
    0xd56fbaba, 0x88f07878, 0x6f4a2525, 0x725c2e2e, 0x24381c1c, 0xf157a6a6, 0xc773b4b4, 0x5197c6c6,
    0x23cbe8e8, 0x7ca1dddd, 0x9ce87474, 0x213e1f1f, 0xdd964b4b, 0xdc61bdbd, 0x860d8b8b, 0x850f8a8a,
    0x90e07070, 0x427c3e3e, 0xc471b5b5, 0xaacc6666, 0xd8904848, 0x05060303, 0x01f7f6f6, 0x121c0e0e,
    0xa3c26161, 0x5f6a3535, 0xf9ae5757, 0xd069b9b9, 0x91178686, 0x5899c1c1, 0x273a1d1d, 0xb9279e9e,
    0x38d9e1e1, 0x13ebf8f8, 0xb32b9898, 0x33221111, 0xbbd26969, 0x70a9d9d9, 0x89078e8e, 0xa7339494,
    0xb62d9b9b, 0x223c1e1e, 0x92158787, 0x20c9e9e9, 0x4987cece, 0xffaa5555, 0x78502828, 0x7aa5dfdf,
    0x8f038c8c, 0xf859a1a1, 0x80098989, 0x171a0d0d, 0xda65bfbf, 0x31d7e6e6, 0xc6844242, 0xb8d06868,
    0xc3824141, 0xb0299999, 0x775a2d2d, 0x111e0f0f, 0xcb7bb0b0, 0xfca85454, 0xd66dbbbb, 0x3a2c1616,
];

const TE2: [u32; 256] = [
    0x63a5c663, 0x7c84f87c, 0x7799ee77, 0x7b8df67b, 0xf20dfff2, 0x6bbdd66b, 0x6fb1de6f, 0xc55491c5,
    0x30506030, 0x01030201, 0x67a9ce67, 0x2b7d562b, 0xfe19e7fe, 0xd762b5d7, 0xabe64dab, 0x769aec76,
    0xca458fca, 0x829d1f82, 0xc94089c9, 0x7d87fa7d, 0xfa15effa, 0x59ebb259, 0x47c98e47, 0xf00bfbf0,
    0xadec41ad, 0xd467b3d4, 0xa2fd5fa2, 0xafea45af, 0x9cbf239c, 0xa4f753a4, 0x7296e472, 0xc05b9bc0,
    0xb7c275b7, 0xfd1ce1fd, 0x93ae3d93, 0x266a4c26, 0x365a6c36, 0x3f417e3f, 0xf702f5f7, 0xcc4f83cc,
    0x345c6834, 0xa5f451a5, 0xe534d1e5, 0xf108f9f1, 0x7193e271, 0xd873abd8, 0x31536231, 0x153f2a15,
    0x040c0804, 0xc75295c7, 0x23654623, 0xc35e9dc3, 0x18283018, 0x96a13796, 0x050f0a05, 0x9ab52f9a,
    0x07090e07, 0x12362412, 0x809b1b80, 0xe23ddfe2, 0xeb26cdeb, 0x27694e27, 0xb2cd7fb2, 0x759fea75,
    0x091b1209, 0x839e1d83, 0x2c74582c, 0x1a2e341a, 0x1b2d361b, 0x6eb2dc6e, 0x5aeeb45a, 0xa0fb5ba0,
    0x52f6a452, 0x3b4d763b, 0xd661b7d6, 0xb3ce7db3, 0x297b5229, 0xe33edde3, 0x2f715e2f, 0x84971384,
    0x53f5a653, 0xd168b9d1, 0x00000000, 0xed2cc1ed, 0x20604020, 0xfc1fe3fc, 0xb1c879b1, 0x5bedb65b,
    0x6abed46a, 0xcb468dcb, 0xbed967be, 0x394b7239, 0x4ade944a, 0x4cd4984c, 0x58e8b058, 0xcf4a85cf,
    0xd06bbbd0, 0xef2ac5ef, 0xaae54faa, 0xfb16edfb, 0x43c58643, 0x4dd79a4d, 0x33556633, 0x85941185,
    0x45cf8a45, 0xf910e9f9, 0x02060402, 0x7f81fe7f, 0x50f0a050, 0x3c44783c, 0x9fba259f, 0xa8e34ba8,
    0x51f3a251, 0xa3fe5da3, 0x40c08040, 0x8f8a058f, 0x92ad3f92, 0x9dbc219d, 0x38487038, 0xf504f1f5,
    0xbcdf63bc, 0xb6c177b6, 0xda75afda, 0x21634221, 0x10302010, 0xff1ae5ff, 0xf30efdf3, 0xd26dbfd2,
    0xcd4c81cd, 0x0c14180c, 0x13352613, 0xec2fc3ec, 0x5fe1be5f, 0x97a23597, 0x44cc8844, 0x17392e17,
    0xc45793c4, 0xa7f255a7, 0x7e82fc7e, 0x3d477a3d, 0x64acc864, 0x5de7ba5d, 0x192b3219, 0x7395e673,
    0x60a0c060, 0x81981981, 0x4fd19e4f, 0xdc7fa3dc, 0x22664422, 0x2a7e542a, 0x90ab3b90, 0x88830b88,
    0x46ca8c46, 0xee29c7ee, 0xb8d36bb8, 0x143c2814, 0xde79a7de, 0x5ee2bc5e, 0x0b1d160b, 0xdb76addb,
    0xe03bdbe0, 0x32566432, 0x3a4e743a, 0x0a1e140a, 0x49db9249, 0x060a0c06, 0x246c4824, 0x5ce4b85c,
    0xc25d9fc2, 0xd36ebdd3, 0xacef43ac, 0x62a6c462, 0x91a83991, 0x95a43195, 0xe437d3e4, 0x798bf279,
    0xe732d5e7, 0xc8438bc8, 0x37596e37, 0x6db7da6d, 0x8d8c018d, 0xd564b1d5, 0x4ed29c4e, 0xa9e049a9,
    0x6cb4d86c, 0x56faac56, 0xf407f3f4, 0xea25cfea, 0x65afca65, 0x7a8ef47a, 0xaee947ae, 0x08181008,
    0xbad56fba, 0x7888f078, 0x256f4a25, 0x2e725c2e, 0x1c24381c, 0xa6f157a6, 0xb4c773b4, 0xc65197c6,
    0xe823cbe8, 0xdd7ca1dd, 0x749ce874, 0x1f213e1f, 0x4bdd964b, 0xbddc61bd, 0x8b860d8b, 0x8a850f8a,
    0x7090e070, 0x3e427c3e, 0xb5c471b5, 0x66aacc66, 0x48d89048, 0x03050603, 0xf601f7f6, 0x0e121c0e,
    0x61a3c261, 0x355f6a35, 0x57f9ae57, 0xb9d069b9, 0x86911786, 0xc15899c1, 0x1d273a1d, 0x9eb9279e,
    0xe138d9e1, 0xf813ebf8, 0x98b32b98, 0x11332211, 0x69bbd269, 0xd970a9d9, 0x8e89078e, 0x94a73394,
    0x9bb62d9b, 0x1e223c1e, 0x87921587, 0xe920c9e9, 0xce4987ce, 0x55ffaa55, 0x28785028, 0xdf7aa5df,
    0x8c8f038c, 0xa1f859a1, 0x89800989, 0x0d171a0d, 0xbfda65bf, 0xe631d7e6, 0x42c68442, 0x68b8d068,
    0x41c38241, 0x99b02999, 0x2d775a2d, 0x0f111e0f, 0xb0cb7bb0, 0x54fca854, 0xbbd66dbb, 0x163a2c16,
];

const TE3: [u32; 256] = [
    0x6363a5c6, 0x7c7c84f8, 0x777799ee, 0x7b7b8df6, 0xf2f20dff, 0x6b6bbdd6, 0x6f6fb1de, 0xc5c55491,
    0x30305060, 0x01010302, 0x6767a9ce, 0x2b2b7d56, 0xfefe19e7, 0xd7d762b5, 0xababe64d, 0x76769aec,
    0xcaca458f, 0x82829d1f, 0xc9c94089, 0x7d7d87fa, 0xfafa15ef, 0x5959ebb2, 0x4747c98e, 0xf0f00bfb,
    0xadadec41, 0xd4d467b3, 0xa2a2fd5f, 0xafafea45, 0x9c9cbf23, 0xa4a4f753, 0x727296e4, 0xc0c05b9b,
    0xb7b7c275, 0xfdfd1ce1, 0x9393ae3d, 0x26266a4c, 0x36365a6c, 0x3f3f417e, 0xf7f702f5, 0xcccc4f83,
    0x34345c68, 0xa5a5f451, 0xe5e534d1, 0xf1f108f9, 0x717193e2, 0xd8d873ab, 0x31315362, 0x15153f2a,
    0x04040c08, 0xc7c75295, 0x23236546, 0xc3c35e9d, 0x18182830, 0x9696a137, 0x05050f0a, 0x9a9ab52f,
    0x0707090e, 0x12123624, 0x80809b1b, 0xe2e23ddf, 0xebeb26cd, 0x2727694e, 0xb2b2cd7f, 0x75759fea,
    0x09091b12, 0x83839e1d, 0x2c2c7458, 0x1a1a2e34, 0x1b1b2d36, 0x6e6eb2dc, 0x5a5aeeb4, 0xa0a0fb5b,
    0x5252f6a4, 0x3b3b4d76, 0xd6d661b7, 0xb3b3ce7d, 0x29297b52, 0xe3e33edd, 0x2f2f715e, 0x84849713,
    0x5353f5a6, 0xd1d168b9, 0x00000000, 0xeded2cc1, 0x20206040, 0xfcfc1fe3, 0xb1b1c879, 0x5b5bedb6,
    0x6a6abed4, 0xcbcb468d, 0xbebed967, 0x39394b72, 0x4a4ade94, 0x4c4cd498, 0x5858e8b0, 0xcfcf4a85,
    0xd0d06bbb, 0xefef2ac5, 0xaaaae54f, 0xfbfb16ed, 0x4343c586, 0x4d4dd79a, 0x33335566, 0x85859411,
    0x4545cf8a, 0xf9f910e9, 0x02020604, 0x7f7f81fe, 0x5050f0a0, 0x3c3c4478, 0x9f9fba25, 0xa8a8e34b,
    0x5151f3a2, 0xa3a3fe5d, 0x4040c080, 0x8f8f8a05, 0x9292ad3f, 0x9d9dbc21, 0x38384870, 0xf5f504f1,
    0xbcbcdf63, 0xb6b6c177, 0xdada75af, 0x21216342, 0x10103020, 0xffff1ae5, 0xf3f30efd, 0xd2d26dbf,
    0xcdcd4c81, 0x0c0c1418, 0x13133526, 0xecec2fc3, 0x5f5fe1be, 0x9797a235, 0x4444cc88, 0x1717392e,
    0xc4c45793, 0xa7a7f255, 0x7e7e82fc, 0x3d3d477a, 0x6464acc8, 0x5d5de7ba, 0x19192b32, 0x737395e6,
    0x6060a0c0, 0x81819819, 0x4f4fd19e, 0xdcdc7fa3, 0x22226644, 0x2a2a7e54, 0x9090ab3b, 0x8888830b,
    0x4646ca8c, 0xeeee29c7, 0xb8b8d36b, 0x14143c28, 0xdede79a7, 0x5e5ee2bc, 0x0b0b1d16, 0xdbdb76ad,
    0xe0e03bdb, 0x32325664, 0x3a3a4e74, 0x0a0a1e14, 0x4949db92, 0x06060a0c, 0x24246c48, 0x5c5ce4b8,
    0xc2c25d9f, 0xd3d36ebd, 0xacacef43, 0x6262a6c4, 0x9191a839, 0x9595a431, 0xe4e437d3, 0x79798bf2,
    0xe7e732d5, 0xc8c8438b, 0x3737596e, 0x6d6db7da, 0x8d8d8c01, 0xd5d564b1, 0x4e4ed29c, 0xa9a9e049,
    0x6c6cb4d8, 0x5656faac, 0xf4f407f3, 0xeaea25cf, 0x6565afca, 0x7a7a8ef4, 0xaeaee947, 0x08081810,
    0xbabad56f, 0x787888f0, 0x25256f4a, 0x2e2e725c, 0x1c1c2438, 0xa6a6f157, 0xb4b4c773, 0xc6c65197,
    0xe8e823cb, 0xdddd7ca1, 0x74749ce8, 0x1f1f213e, 0x4b4bdd96, 0xbdbddc61, 0x8b8b860d, 0x8a8a850f,
    0x707090e0, 0x3e3e427c, 0xb5b5c471, 0x6666aacc, 0x4848d890, 0x03030506, 0xf6f601f7, 0x0e0e121c,
    0x6161a3c2, 0x35355f6a, 0x5757f9ae, 0xb9b9d069, 0x86869117, 0xc1c15899, 0x1d1d273a, 0x9e9eb927,
    0xe1e138d9, 0xf8f813eb, 0x9898b32b, 0x11113322, 0x6969bbd2, 0xd9d970a9, 0x8e8e8907, 0x9494a733,
    0x9b9bb62d, 0x1e1e223c, 0x87879215, 0xe9e920c9, 0xcece4987, 0x5555ffaa, 0x28287850, 0xdfdf7aa5,
    0x8c8c8f03, 0xa1a1f859, 0x89898009, 0x0d0d171a, 0xbfbfda65, 0xe6e631d7, 0x4242c684, 0x6868b8d0,
    0x4141c382, 0x9999b029, 0x2d2d775a, 0x0f0f111e, 0xb0b0cb7b, 0x5454fca8, 0xbbbbd66d, 0x16163a2c,
];

const TD0: [u32; 256] = [
    0x51f4a750, 0x7e416553, 0x1a17a4c3, 0x3a275e96, 0x3bab6bcb, 0x1f9d45f1, 0xacfa58ab, 0x4be30393,
    0x2030fa55, 0xad766df6, 0x88cc7691, 0xf5024c25, 0x4fe5d7fc, 0xc52acbd7, 0x26354480, 0xb562a38f,
    0xdeb15a49, 0x25ba1b67, 0x45ea0e98, 0x5dfec0e1, 0xc32f7502, 0x814cf012, 0x8d4697a3, 0x6bd3f9c6,
    0x038f5fe7, 0x15929c95, 0xbf6d7aeb, 0x955259da, 0xd4be832d, 0x587421d3, 0x49e06929, 0x8ec9c844,
    0x75c2896a, 0xf48e7978, 0x99583e6b, 0x27b971dd, 0xbee14fb6, 0xf088ad17, 0xc920ac66, 0x7dce3ab4,
    0x63df4a18, 0xe51a3182, 0x97513360, 0x62537f45, 0xb16477e0, 0xbb6bae84, 0xfe81a01c, 0xf9082b94,
    0x70486858, 0x8f45fd19, 0x94de6c87, 0x527bf8b7, 0xab73d323, 0x724b02e2, 0xe31f8f57, 0x6655ab2a,
    0xb2eb2807, 0x2fb5c203, 0x86c57b9a, 0xd33708a5, 0x302887f2, 0x23bfa5b2, 0x02036aba, 0xed16825c,
    0x8acf1c2b, 0xa779b492, 0xf307f2f0, 0x4e69e2a1, 0x65daf4cd, 0x0605bed5, 0xd134621f, 0xc4a6fe8a,
    0x342e539d, 0xa2f355a0, 0x058ae132, 0xa4f6eb75, 0x0b83ec39, 0x4060efaa, 0x5e719f06, 0xbd6e1051,
    0x3e218af9, 0x96dd063d, 0xdd3e05ae, 0x4de6bd46, 0x91548db5, 0x71c45d05, 0x0406d46f, 0x605015ff,
    0x1998fb24, 0xd6bde997, 0x894043cc, 0x67d99e77, 0xb0e842bd, 0x07898b88, 0xe7195b38, 0x79c8eedb,
    0xa17c0a47, 0x7c420fe9, 0xf8841ec9, 0x00000000, 0x09808683, 0x322bed48, 0x1e1170ac, 0x6c5a724e,
    0xfd0efffb, 0x0f853856, 0x3daed51e, 0x362d3927, 0x0a0fd964, 0x685ca621, 0x9b5b54d1, 0x24362e3a,
    0x0c0a67b1, 0x9357e70f, 0xb4ee96d2, 0x1b9b919e, 0x80c0c54f, 0x61dc20a2, 0x5a774b69, 0x1c121a16,
    0xe293ba0a, 0xc0a02ae5, 0x3c22e043, 0x121b171d, 0x0e090d0b, 0xf28bc7ad, 0x2db6a8b9, 0x141ea9c8,
    0x57f11985, 0xaf75074c, 0xee99ddbb, 0xa37f60fd, 0xf701269f, 0x5c72f5bc, 0x44663bc5, 0x5bfb7e34,
    0x8b432976, 0xcb23c6dc, 0xb6edfc68, 0xb8e4f163, 0xd731dcca, 0x42638510, 0x13972240, 0x84c61120,
    0x854a247d, 0xd2bb3df8, 0xaef93211, 0xc729a16d, 0x1d9e2f4b, 0xdcb230f3, 0x0d8652ec, 0x77c1e3d0,
    0x2bb3166c, 0xa970b999, 0x119448fa, 0x47e96422, 0xa8fc8cc4, 0xa0f03f1a, 0x567d2cd8, 0x223390ef,
    0x87494ec7, 0xd938d1c1, 0x8ccaa2fe, 0x98d40b36, 0xa6f581cf, 0xa57ade28, 0xdab78e26, 0x3fadbfa4,
    0x2c3a9de4, 0x5078920d, 0x6a5fcc9b, 0x547e4662, 0xf68d13c2, 0x90d8b8e8, 0x2e39f75e, 0x82c3aff5,
    0x9f5d80be, 0x69d0937c, 0x6fd52da9, 0xcf2512b3, 0xc8ac993b, 0x10187da7, 0xe89c636e, 0xdb3bbb7b,
    0xcd267809, 0x6e5918f4, 0xec9ab701, 0x834f9aa8, 0xe6956e65, 0xaaffe67e, 0x21bccf08, 0xef15e8e6,
    0xbae79bd9, 0x4a6f36ce, 0xea9f09d4, 0x29b07cd6, 0x31a4b2af, 0x2a3f2331, 0xc6a59430, 0x35a266c0,
    0x744ebc37, 0xfc82caa6, 0xe090d0b0, 0x33a7d815, 0xf104984a, 0x41ecdaf7, 0x7fcd500e, 0x1791f62f,
    0x764dd68d, 0x43efb04d, 0xccaa4d54, 0xe49604df, 0x9ed1b5e3, 0x4c6a881b, 0xc12c1fb8, 0x4665517f,
    0x9d5eea04, 0x018c355d, 0xfa877473, 0xfb0b412e, 0xb3671d5a, 0x92dbd252, 0xe9105633, 0x6dd64713,
    0x9ad7618c, 0x37a10c7a, 0x59f8148e, 0xeb133c89, 0xcea927ee, 0xb761c935, 0xe11ce5ed, 0x7a47b13c,
    0x9cd2df59, 0x55f2733f, 0x1814ce79, 0x73c737bf, 0x53f7cdea, 0x5ffdaa5b, 0xdf3d6f14, 0x7844db86,
    0xcaaff381, 0xb968c43e, 0x3824342c, 0xc2a3405f, 0x161dc372, 0xbce2250c, 0x283c498b, 0xff0d9541,
    0x39a80171, 0x080cb3de, 0xd8b4e49c, 0x6456c190, 0x7bcb8461, 0xd532b670, 0x486c5c74, 0xd0b85742,
];

const TD1: [u32; 256] = [
    0x5051f4a7, 0x537e4165, 0xc31a17a4, 0x963a275e, 0xcb3bab6b, 0xf11f9d45, 0xabacfa58, 0x934be303,
    0x552030fa, 0xf6ad766d, 0x9188cc76, 0x25f5024c, 0xfc4fe5d7, 0xd7c52acb, 0x80263544, 0x8fb562a3,
    0x49deb15a, 0x6725ba1b, 0x9845ea0e, 0xe15dfec0, 0x02c32f75, 0x12814cf0, 0xa38d4697, 0xc66bd3f9,
    0xe7038f5f, 0x9515929c, 0xebbf6d7a, 0xda955259, 0x2dd4be83, 0xd3587421, 0x2949e069, 0x448ec9c8,
    0x6a75c289, 0x78f48e79, 0x6b99583e, 0xdd27b971, 0xb6bee14f, 0x17f088ad, 0x66c920ac, 0xb47dce3a,
    0x1863df4a, 0x82e51a31, 0x60975133, 0x4562537f, 0xe0b16477, 0x84bb6bae, 0x1cfe81a0, 0x94f9082b,
    0x58704868, 0x198f45fd, 0x8794de6c, 0xb7527bf8, 0x23ab73d3, 0xe2724b02, 0x57e31f8f, 0x2a6655ab,
    0x07b2eb28, 0x032fb5c2, 0x9a86c57b, 0xa5d33708, 0xf2302887, 0xb223bfa5, 0xba02036a, 0x5ced1682,
    0x2b8acf1c, 0x92a779b4, 0xf0f307f2, 0xa14e69e2, 0xcd65daf4, 0xd50605be, 0x1fd13462, 0x8ac4a6fe,
    0x9d342e53, 0xa0a2f355, 0x32058ae1, 0x75a4f6eb, 0x390b83ec, 0xaa4060ef, 0x065e719f, 0x51bd6e10,
    0xf93e218a, 0x3d96dd06, 0xaedd3e05, 0x464de6bd, 0xb591548d, 0x0571c45d, 0x6f0406d4, 0xff605015,
    0x241998fb, 0x97d6bde9, 0xcc894043, 0x7767d99e, 0xbdb0e842, 0x8807898b, 0x38e7195b, 0xdb79c8ee,
    0x47a17c0a, 0xe97c420f, 0xc9f8841e, 0x00000000, 0x83098086, 0x48322bed, 0xac1e1170, 0x4e6c5a72,
    0xfbfd0eff, 0x560f8538, 0x1e3daed5, 0x27362d39, 0x640a0fd9, 0x21685ca6, 0xd19b5b54, 0x3a24362e,
    0xb10c0a67, 0x0f9357e7, 0xd2b4ee96, 0x9e1b9b91, 0x4f80c0c5, 0xa261dc20, 0x695a774b, 0x161c121a,
    0x0ae293ba, 0xe5c0a02a, 0x433c22e0, 0x1d121b17, 0x0b0e090d, 0xadf28bc7, 0xb92db6a8, 0xc8141ea9,
    0x8557f119, 0x4caf7507, 0xbbee99dd, 0xfda37f60, 0x9ff70126, 0xbc5c72f5, 0xc544663b, 0x345bfb7e,
    0x768b4329, 0xdccb23c6, 0x68b6edfc, 0x63b8e4f1, 0xcad731dc, 0x10426385, 0x40139722, 0x2084c611,
    0x7d854a24, 0xf8d2bb3d, 0x11aef932, 0x6dc729a1, 0x4b1d9e2f, 0xf3dcb230, 0xec0d8652, 0xd077c1e3,
    0x6c2bb316, 0x99a970b9, 0xfa119448, 0x2247e964, 0xc4a8fc8c, 0x1aa0f03f, 0xd8567d2c, 0xef223390,
    0xc787494e, 0xc1d938d1, 0xfe8ccaa2, 0x3698d40b, 0xcfa6f581, 0x28a57ade, 0x26dab78e, 0xa43fadbf,
    0xe42c3a9d, 0x0d507892, 0x9b6a5fcc, 0x62547e46, 0xc2f68d13, 0xe890d8b8, 0x5e2e39f7, 0xf582c3af,
    0xbe9f5d80, 0x7c69d093, 0xa96fd52d, 0xb3cf2512, 0x3bc8ac99, 0xa710187d, 0x6ee89c63, 0x7bdb3bbb,
    0x09cd2678, 0xf46e5918, 0x01ec9ab7, 0xa8834f9a, 0x65e6956e, 0x7eaaffe6, 0x0821bccf, 0xe6ef15e8,
    0xd9bae79b, 0xce4a6f36, 0xd4ea9f09, 0xd629b07c, 0xaf31a4b2, 0x312a3f23, 0x30c6a594, 0xc035a266,
    0x37744ebc, 0xa6fc82ca, 0xb0e090d0, 0x1533a7d8, 0x4af10498, 0xf741ecda, 0x0e7fcd50, 0x2f1791f6,
    0x8d764dd6, 0x4d43efb0, 0x54ccaa4d, 0xdfe49604, 0xe39ed1b5, 0x1b4c6a88, 0xb8c12c1f, 0x7f466551,
    0x049d5eea, 0x5d018c35, 0x73fa8774, 0x2efb0b41, 0x5ab3671d, 0x5292dbd2, 0x33e91056, 0x136dd647,
    0x8c9ad761, 0x7a37a10c, 0x8e59f814, 0x89eb133c, 0xeecea927, 0x35b761c9, 0xede11ce5, 0x3c7a47b1,
    0x599cd2df, 0x3f55f273, 0x791814ce, 0xbf73c737, 0xea53f7cd, 0x5b5ffdaa, 0x14df3d6f, 0x867844db,
    0x81caaff3, 0x3eb968c4, 0x2c382434, 0x5fc2a340, 0x72161dc3, 0x0cbce225, 0x8b283c49, 0x41ff0d95,
    0x7139a801, 0xde080cb3, 0x9cd8b4e4, 0x906456c1, 0x617bcb84, 0x70d532b6, 0x74486c5c, 0x42d0b857,
];

const TD2: [u32; 256] = [
    0xa75051f4, 0x65537e41, 0xa4c31a17, 0x5e963a27, 0x6bcb3bab, 0x45f11f9d, 0x58abacfa, 0x03934be3,
    0xfa552030, 0x6df6ad76, 0x769188cc, 0x4c25f502, 0xd7fc4fe5, 0xcbd7c52a, 0x44802635, 0xa38fb562,
    0x5a49deb1, 0x1b6725ba, 0x0e9845ea, 0xc0e15dfe, 0x7502c32f, 0xf012814c, 0x97a38d46, 0xf9c66bd3,
    0x5fe7038f, 0x9c951592, 0x7aebbf6d, 0x59da9552, 0x832dd4be, 0x21d35874, 0x692949e0, 0xc8448ec9,
    0x896a75c2, 0x7978f48e, 0x3e6b9958, 0x71dd27b9, 0x4fb6bee1, 0xad17f088, 0xac66c920, 0x3ab47dce,
    0x4a1863df, 0x3182e51a, 0x33609751, 0x7f456253, 0x77e0b164, 0xae84bb6b, 0xa01cfe81, 0x2b94f908,
    0x68587048, 0xfd198f45, 0x6c8794de, 0xf8b7527b, 0xd323ab73, 0x02e2724b, 0x8f57e31f, 0xab2a6655,
    0x2807b2eb, 0xc2032fb5, 0x7b9a86c5, 0x08a5d337, 0x87f23028, 0xa5b223bf, 0x6aba0203, 0x825ced16,
    0x1c2b8acf, 0xb492a779, 0xf2f0f307, 0xe2a14e69, 0xf4cd65da, 0xbed50605, 0x621fd134, 0xfe8ac4a6,
    0x539d342e, 0x55a0a2f3, 0xe132058a, 0xeb75a4f6, 0xec390b83, 0xefaa4060, 0x9f065e71, 0x1051bd6e,
    0x8af93e21, 0x063d96dd, 0x05aedd3e, 0xbd464de6, 0x8db59154, 0x5d0571c4, 0xd46f0406, 0x15ff6050,
    0xfb241998, 0xe997d6bd, 0x43cc8940, 0x9e7767d9, 0x42bdb0e8, 0x8b880789, 0x5b38e719, 0xeedb79c8,
    0x0a47a17c, 0x0fe97c42, 0x1ec9f884, 0x00000000, 0x86830980, 0xed48322b, 0x70ac1e11, 0x724e6c5a,
    0xfffbfd0e, 0x38560f85, 0xd51e3dae, 0x3927362d, 0xd9640a0f, 0xa621685c, 0x54d19b5b, 0x2e3a2436,
    0x67b10c0a, 0xe70f9357, 0x96d2b4ee, 0x919e1b9b, 0xc54f80c0, 0x20a261dc, 0x4b695a77, 0x1a161c12,
    0xba0ae293, 0x2ae5c0a0, 0xe0433c22, 0x171d121b, 0x0d0b0e09, 0xc7adf28b, 0xa8b92db6, 0xa9c8141e,
    0x198557f1, 0x074caf75, 0xddbbee99, 0x60fda37f, 0x269ff701, 0xf5bc5c72, 0x3bc54466, 0x7e345bfb,
    0x29768b43, 0xc6dccb23, 0xfc68b6ed, 0xf163b8e4, 0xdccad731, 0x85104263, 0x22401397, 0x112084c6,
    0x247d854a, 0x3df8d2bb, 0x3211aef9, 0xa16dc729, 0x2f4b1d9e, 0x30f3dcb2, 0x52ec0d86, 0xe3d077c1,
    0x166c2bb3, 0xb999a970, 0x48fa1194, 0x642247e9, 0x8cc4a8fc, 0x3f1aa0f0, 0x2cd8567d, 0x90ef2233,
    0x4ec78749, 0xd1c1d938, 0xa2fe8cca, 0x0b3698d4, 0x81cfa6f5, 0xde28a57a, 0x8e26dab7, 0xbfa43fad,
    0x9de42c3a, 0x920d5078, 0xcc9b6a5f, 0x4662547e, 0x13c2f68d, 0xb8e890d8, 0xf75e2e39, 0xaff582c3,
    0x80be9f5d, 0x937c69d0, 0x2da96fd5, 0x12b3cf25, 0x993bc8ac, 0x7da71018, 0x636ee89c, 0xbb7bdb3b,
    0x7809cd26, 0x18f46e59, 0xb701ec9a, 0x9aa8834f, 0x6e65e695, 0xe67eaaff, 0xcf0821bc, 0xe8e6ef15,
    0x9bd9bae7, 0x36ce4a6f, 0x09d4ea9f, 0x7cd629b0, 0xb2af31a4, 0x23312a3f, 0x9430c6a5, 0x66c035a2,
    0xbc37744e, 0xcaa6fc82, 0xd0b0e090, 0xd81533a7, 0x984af104, 0xdaf741ec, 0x500e7fcd, 0xf62f1791,
    0xd68d764d, 0xb04d43ef, 0x4d54ccaa, 0x04dfe496, 0xb5e39ed1, 0x881b4c6a, 0x1fb8c12c, 0x517f4665,
    0xea049d5e, 0x355d018c, 0x7473fa87, 0x412efb0b, 0x1d5ab367, 0xd25292db, 0x5633e910, 0x47136dd6,
    0x618c9ad7, 0x0c7a37a1, 0x148e59f8, 0x3c89eb13, 0x27eecea9, 0xc935b761, 0xe5ede11c, 0xb13c7a47,
    0xdf599cd2, 0x733f55f2, 0xce791814, 0x37bf73c7, 0xcdea53f7, 0xaa5b5ffd, 0x6f14df3d, 0xdb867844,
    0xf381caaf, 0xc43eb968, 0x342c3824, 0x405fc2a3, 0xc372161d, 0x250cbce2, 0x498b283c, 0x9541ff0d,
    0x017139a8, 0xb3de080c, 0xe49cd8b4, 0xc1906456, 0x84617bcb, 0xb670d532, 0x5c74486c, 0x5742d0b8,
];
const TD3: [u32; 256] = [
    0xf4a75051, 0x4165537e, 0x17a4c31a, 0x275e963a, 0xab6bcb3b, 0x9d45f11f, 0xfa58abac, 0xe303934b,
    0x30fa5520, 0x766df6ad, 0xcc769188, 0x024c25f5, 0xe5d7fc4f, 0x2acbd7c5, 0x35448026, 0x62a38fb5,
    0xb15a49de, 0xba1b6725, 0xea0e9845, 0xfec0e15d, 0x2f7502c3, 0x4cf01281, 0x4697a38d, 0xd3f9c66b,
    0x8f5fe703, 0x929c9515, 0x6d7aebbf, 0x5259da95, 0xbe832dd4, 0x7421d358, 0xe0692949, 0xc9c8448e,
    0xc2896a75, 0x8e7978f4, 0x583e6b99, 0xb971dd27, 0xe14fb6be, 0x88ad17f0, 0x20ac66c9, 0xce3ab47d,
    0xdf4a1863, 0x1a3182e5, 0x51336097, 0x537f4562, 0x6477e0b1, 0x6bae84bb, 0x81a01cfe, 0x082b94f9,
    0x48685870, 0x45fd198f, 0xde6c8794, 0x7bf8b752, 0x73d323ab, 0x4b02e272, 0x1f8f57e3, 0x55ab2a66,
    0xeb2807b2, 0xb5c2032f, 0xc57b9a86, 0x3708a5d3, 0x2887f230, 0xbfa5b223, 0x036aba02, 0x16825ced,
    0xcf1c2b8a, 0x79b492a7, 0x07f2f0f3, 0x69e2a14e, 0xdaf4cd65, 0x05bed506, 0x34621fd1, 0xa6fe8ac4,
    0x2e539d34, 0xf355a0a2, 0x8ae13205, 0xf6eb75a4, 0x83ec390b, 0x60efaa40, 0x719f065e, 0x6e1051bd,
    0x218af93e, 0xdd063d96, 0x3e05aedd, 0xe6bd464d, 0x548db591, 0xc45d0571, 0x06d46f04, 0x5015ff60,
    0x98fb2419, 0xbde997d6, 0x4043cc89, 0xd99e7767, 0xe842bdb0, 0x898b8807, 0x195b38e7, 0xc8eedb79,
    0x7c0a47a1, 0x420fe97c, 0x841ec9f8, 0x00000000, 0x80868309, 0x2bed4832, 0x1170ac1e, 0x5a724e6c,
    0x0efffbfd, 0x8538560f, 0xaed51e3d, 0x2d392736, 0x0fd9640a, 0x5ca62168, 0x5b54d19b, 0x362e3a24,
    0x0a67b10c, 0x57e70f93, 0xee96d2b4, 0x9b919e1b, 0xc0c54f80, 0xdc20a261, 0x774b695a, 0x121a161c,
    0x93ba0ae2, 0xa02ae5c0, 0x22e0433c, 0x1b171d12, 0x090d0b0e, 0x8bc7adf2, 0xb6a8b92d, 0x1ea9c814,
    0xf1198557, 0x75074caf, 0x99ddbbee, 0x7f60fda3, 0x01269ff7, 0x72f5bc5c, 0x663bc544, 0xfb7e345b,
    0x4329768b, 0x23c6dccb, 0xedfc68b6, 0xe4f163b8, 0x31dccad7, 0x63851042, 0x97224013, 0xc6112084,
    0x4a247d85, 0xbb3df8d2, 0xf93211ae, 0x29a16dc7, 0x9e2f4b1d, 0xb230f3dc, 0x8652ec0d, 0xc1e3d077,
    0xb3166c2b, 0x70b999a9, 0x9448fa11, 0xe9642247, 0xfc8cc4a8, 0xf03f1aa0, 0x7d2cd856, 0x3390ef22,
    0x494ec787, 0x38d1c1d9, 0xcaa2fe8c, 0xd40b3698, 0xf581cfa6, 0x7ade28a5, 0xb78e26da, 0xadbfa43f,
    0x3a9de42c, 0x78920d50, 0x5fcc9b6a, 0x7e466254, 0x8d13c2f6, 0xd8b8e890, 0x39f75e2e, 0xc3aff582,
    0x5d80be9f, 0xd0937c69, 0xd52da96f, 0x2512b3cf, 0xac993bc8, 0x187da710, 0x9c636ee8, 0x3bbb7bdb,
    0x267809cd, 0x5918f46e, 0x9ab701ec, 0x4f9aa883, 0x956e65e6, 0xffe67eaa, 0xbccf0821, 0x15e8e6ef,
    0xe79bd9ba, 0x6f36ce4a, 0x9f09d4ea, 0xb07cd629, 0xa4b2af31, 0x3f23312a, 0xa59430c6, 0xa266c035,
    0x4ebc3774, 0x82caa6fc, 0x90d0b0e0, 0xa7d81533, 0x04984af1, 0xecdaf741, 0xcd500e7f, 0x91f62f17,
    0x4dd68d76, 0xefb04d43, 0xaa4d54cc, 0x9604dfe4, 0xd1b5e39e, 0x6a881b4c, 0x2c1fb8c1, 0x65517f46,
    0x5eea049d, 0x8c355d01, 0x877473fa, 0x0b412efb, 0x671d5ab3, 0xdbd25292, 0x105633e9, 0xd647136d,
    0xd7618c9a, 0xa10c7a37, 0xf8148e59, 0x133c89eb, 0xa927eece, 0x61c935b7, 0x1ce5ede1, 0x47b13c7a,
    0xd2df599c, 0xf2733f55, 0x14ce7918, 0xc737bf73, 0xf7cdea53, 0xfdaa5b5f, 0x3d6f14df, 0x44db8678,
    0xaff381ca, 0x68c43eb9, 0x24342c38, 0xa3405fc2, 0x1dc37216, 0xe2250cbc, 0x3c498b28, 0x0d9541ff,
    0xa8017139, 0x0cb3de08, 0xb4e49cd8, 0x56c19064, 0xcb84617b, 0x32b670d5, 0x6c5c7448, 0xb85742d0,
];

const RCON: [u32; 10] = [
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000, 0x80000000,
    0x1B000000,
    0x36000000, /* for 128-bit blocks, Rijndael never uses more than 10 rcon values */
];

#[derive(Clone, Copy, Debug)]
enum AesByte {
    Byte0 = 0,
    Byte1 = 1,
    Byte2 = 2,
    Byte3 = 3,
}

#[derive(Debug)]
pub struct Aes {
    // Why does this need to be so large?
    key: [u32; 60],
    rounds: usize,
}

impl Default for Aes {
    fn default() -> Self {
        Aes {
            key: [0u32; 60],
            rounds: 0,
        }
    }
}

fn aes_enc_round(arg1: u32, arg2: u32, id: AesByte) -> u32 {
    extern "C" {
        fn vex_aes_enc_id_0(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_1(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_2(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_3(arg1: u32, arg2: u32) -> u32;
    }
    match id {
        AesByte::Byte0 => unsafe { vex_aes_enc_id_0(arg1, arg2) },
        AesByte::Byte1 => unsafe { vex_aes_enc_id_1(arg1, arg2) },
        AesByte::Byte2 => unsafe { vex_aes_enc_id_2(arg1, arg2) },
        AesByte::Byte3 => unsafe { vex_aes_enc_id_3(arg1, arg2) },
    }
}

fn aes_enc_round_last(arg1: u32, arg2: u32, id: AesByte) -> u32 {
    extern "C" {
        fn vex_aes_enc_id_last_0(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_last_1(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_last_2(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_enc_id_last_3(arg1: u32, arg2: u32) -> u32;
    }
    match id {
        AesByte::Byte0 => unsafe { vex_aes_enc_id_last_0(arg1, arg2) },
        AesByte::Byte1 => unsafe { vex_aes_enc_id_last_1(arg1, arg2) },
        AesByte::Byte2 => unsafe { vex_aes_enc_id_last_2(arg1, arg2) },
        AesByte::Byte3 => unsafe { vex_aes_enc_id_last_3(arg1, arg2) },
    }
}

fn aes_dec_round(arg1: u32, arg2: u32, id: AesByte) -> u32 {
    extern "C" {
        fn vex_aes_dec_id_0(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_1(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_2(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_3(arg1: u32, arg2: u32) -> u32;
    }
    match id {
        AesByte::Byte0 => unsafe { vex_aes_dec_id_0(arg1, arg2) },
        AesByte::Byte1 => unsafe { vex_aes_dec_id_1(arg1, arg2) },
        AesByte::Byte2 => unsafe { vex_aes_dec_id_2(arg1, arg2) },
        AesByte::Byte3 => unsafe { vex_aes_dec_id_3(arg1, arg2) },
    }
}

fn aes_dec_round_last(arg1: u32, arg2: u32, id: AesByte) -> u32 {
    extern "C" {
        fn vex_aes_dec_id_last_0(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_last_1(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_last_2(arg1: u32, arg2: u32) -> u32;
        fn vex_aes_dec_id_last_3(arg1: u32, arg2: u32) -> u32;
    }
    match id {
        AesByte::Byte0 => unsafe { vex_aes_dec_id_last_0(arg1, arg2) },
        AesByte::Byte1 => unsafe { vex_aes_dec_id_last_1(arg1, arg2) },
        AesByte::Byte2 => unsafe { vex_aes_dec_id_last_2(arg1, arg2) },
        AesByte::Byte3 => unsafe { vex_aes_dec_id_last_3(arg1, arg2) },
    }
}

use core::convert::TryInto;

fn get_u32_be(input: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes(input[offset..offset+4].try_into().unwrap())
}

fn get_u32_le(input: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(input[offset..offset+4].try_into().unwrap())
}

fn set_u32(output: &mut [u8], offset: usize, value: u32) {
    let tmp = value.to_le_bytes();
    output[offset + 0] = tmp[0];
    output[offset + 1] = tmp[1];
    output[offset + 2] = tmp[2];
    output[offset + 3] = tmp[3];
}

pub fn set_encrypt_key(user_key: &[u8], key: &mut Aes) -> Result<(), &'static str> {
    set_encrypt_key_inner(user_key, key, true)
}

fn set_encrypt_key_inner(
    user_key: &[u8],
    key: &mut Aes,
    swap_final: bool,
) -> Result<(), &'static str> {
    let rk = &mut key.key;

    key.rounds = match user_key.len() {
        16 => 10,
        24 => 12,
        32 => 14,
        _ => Err("unrecognized keylength")?,
    };

    rk[0] = get_u32_be(user_key, 0);
    rk[1] = get_u32_be(user_key, 4);
    rk[2] = get_u32_be(user_key, 8);
    rk[3] = get_u32_be(user_key, 12);
    let mut rk_offset = 0;
    if user_key.len() == 16 {
        for i in 0..10 {
            let temp = rk[3 + rk_offset] as usize;
            rk[4 + rk_offset] = rk[0 + rk_offset]
                ^ (TE2[(temp >> 16) & 0xff] & 0xff000000)
                ^ (TE3[(temp >> 8) & 0xff] & 0x00ff0000)
                ^ (TE0[(temp) & 0xff] & 0x0000ff00)
                ^ (TE1[(temp >> 24)] & 0x000000ff)
                ^ RCON[i];
            rk[5 + rk_offset] = rk[1 + rk_offset] ^ rk[4 + rk_offset];
            rk[6 + rk_offset] = rk[2 + rk_offset] ^ rk[5 + rk_offset];
            rk[7 + rk_offset] = rk[3 + rk_offset] ^ rk[6 + rk_offset];
            rk_offset += 4;
        }
        if swap_final {
            for value in &mut key.key {
                *value = value.swap_bytes();
            }
        }
        return Ok(());
    }
    rk[4] = get_u32_be(user_key, 16);
    rk[5] = get_u32_be(user_key, 20);
    if user_key.len() == 24 {
        let mut rk_offset = 0;
        for i in 0.. {
            let temp = rk[5 + rk_offset] as usize;
            rk[6 + rk_offset] = rk[0 + rk_offset]
                ^ (TE2[(temp >> 16) & 0xff] & 0xff000000)
                ^ (TE3[(temp >> 8) & 0xff] & 0x00ff0000)
                ^ (TE0[(temp) & 0xff] & 0x0000ff00)
                ^ (TE1[(temp >> 24)] & 0x000000ff)
                ^ RCON[i];
            rk[7 + rk_offset] = rk[1 + rk_offset] ^ rk[6 + rk_offset];
            rk[8 + rk_offset] = rk[2 + rk_offset] ^ rk[7 + rk_offset];
            rk[9 + rk_offset] = rk[3 + rk_offset] ^ rk[8 + rk_offset];
            if i == 7 {
                break;
            }
            rk[10 + rk_offset] = rk[4 + rk_offset] ^ rk[9 + rk_offset];
            rk[11 + rk_offset] = rk[5 + rk_offset] ^ rk[10 + rk_offset];
            rk_offset += 6;
        }
        if swap_final {
            for value in &mut key.key {
                *value = value.swap_bytes();
            }
        }
        return Ok(());
    }
    rk[6] = get_u32_be(user_key, 24);
    rk[7] = get_u32_be(user_key, 28);
    if user_key.len() == 32 {
        let mut rk_offset = 0;
        for i in 0.. {
            let temp = rk[7 + rk_offset] as usize;
            rk[8 + rk_offset] = rk[0 + rk_offset]
                ^ (TE2[(temp >> 16) & 0xff] & 0xff000000)
                ^ (TE3[(temp >> 8) & 0xff] & 0x00ff0000)
                ^ (TE0[(temp) & 0xff] & 0x0000ff00)
                ^ (TE1[(temp >> 24)] & 0x000000ff)
                ^ RCON[i];
            rk[9 + rk_offset] = rk[1 + rk_offset] ^ rk[8 + rk_offset];
            rk[10 + rk_offset] = rk[2 + rk_offset] ^ rk[9 + rk_offset];
            rk[11 + rk_offset] = rk[3 + rk_offset] ^ rk[10 + rk_offset];

            if i == 6 {
                break;
            }
            let temp = rk[11 + rk_offset] as usize;
            rk[12 + rk_offset] = rk[4 + rk_offset]
                ^ (TE2[(temp >> 24)] & 0xff000000)
                ^ (TE3[(temp >> 16) & 0xff] & 0x00ff0000)
                ^ (TE0[(temp >> 8) & 0xff] & 0x0000ff00)
                ^ (TE1[(temp) & 0xff] & 0x000000ff);
            rk[13 + rk_offset] = rk[5 + rk_offset] ^ rk[12 + rk_offset];
            rk[14 + rk_offset] = rk[6 + rk_offset] ^ rk[13 + rk_offset];
            rk[15 + rk_offset] = rk[7 + rk_offset] ^ rk[14 + rk_offset];

            rk_offset += 8;
        }
        if swap_final {
            for value in &mut key.key {
                *value = value.swap_bytes();
            }
        }
        return Ok(());
    }
    unreachable!();
}

pub fn set_decrypt_key(user_key: &[u8], key: &mut Aes) -> Result<(), &'static str> {
    set_encrypt_key_inner(user_key, key, false)?;
    let rk = &mut key.key;

    /* invert the order of the round keys: */
    let mut i = 0;
    let mut j = 4 * key.rounds;
    while i < j {
        let temp = rk[i];
        rk[i] = rk[j];
        rk[j] = temp;

        let temp = rk[i + 1];
        rk[i + 1] = rk[j + 1];
        rk[j + 1] = temp;

        let temp = rk[i + 2];
        rk[i + 2] = rk[j + 2];
        rk[j + 2] = temp;

        let temp = rk[i + 3];
        rk[i + 3] = rk[j + 3];
        rk[j + 3] = temp;

        i += 4;
        j -= 4;
    }

    /* apply the inverse MixColumn transform to all round keys but the first and the last: */
    let mut rk_offset = 4;
    for _ in 1..key.rounds {
        rk[0 + rk_offset] = TD0[TE1[(rk[0 + rk_offset] as usize >> 24)] as usize & 0xff]
            ^ TD1[TE1[(rk[0 + rk_offset] as usize >> 16) & 0xff] as usize & 0xff]
            ^ TD2[TE1[(rk[0 + rk_offset] as usize >> 8) & 0xff] as usize & 0xff]
            ^ TD3[TE1[(rk[0 + rk_offset] as usize) & 0xff] as usize & 0xff];
        rk[1 + rk_offset] = TD0[TE1[(rk[1 + rk_offset] as usize >> 24)] as usize & 0xff]
            ^ TD1[TE1[(rk[1 + rk_offset] as usize >> 16) & 0xff] as usize & 0xff]
            ^ TD2[TE1[(rk[1 + rk_offset] as usize >> 8) & 0xff] as usize & 0xff]
            ^ TD3[TE1[(rk[1 + rk_offset] as usize) & 0xff] as usize & 0xff];
        rk[2 + rk_offset] = TD0[TE1[(rk[2 + rk_offset] as usize >> 24)] as usize & 0xff]
            ^ TD1[TE1[(rk[2 + rk_offset] as usize >> 16) & 0xff] as usize & 0xff]
            ^ TD2[TE1[(rk[2 + rk_offset] as usize >> 8) & 0xff] as usize & 0xff]
            ^ TD3[TE1[(rk[2 + rk_offset] as usize) & 0xff] as usize & 0xff];
        rk[3 + rk_offset] = TD0[TE1[(rk[3 + rk_offset] as usize >> 24)] as usize & 0xff]
            ^ TD1[TE1[(rk[3 + rk_offset] as usize >> 16) & 0xff] as usize & 0xff]
            ^ TD2[TE1[(rk[3 + rk_offset] as usize >> 8) & 0xff] as usize & 0xff]
            ^ TD3[TE1[(rk[3 + rk_offset] as usize) & 0xff] as usize & 0xff];
        rk_offset += 4;
    }

    for value in &mut key.key {
        *value = value.swap_bytes();
    }
    Ok(())
}

pub fn vexriscv_aes_encrypt(input: &[u8], output: &mut [u8], key: &Aes) {
    let rk = key.key;

    // We do two rounds per loop
    let mut round_count = key.rounds / 2;

    let mut s0 = get_u32_le(input, 0);
    let mut s1 = get_u32_le(input, 4);
    let mut s2 = get_u32_le(input, 8);
    let mut s3 = get_u32_le(input, 12);

    let mut t0 = rk[0];
    let mut t1 = rk[1];
    let mut t2 = rk[2];
    let mut t3 = rk[3];

    s0 ^= t0;
    s1 ^= t1;
    s2 ^= t2;
    s3 ^= t3;

    let mut rk_offset = 0;
    loop {
        t0 = rk[rk_offset + 4];
        t1 = rk[rk_offset + 5];
        t2 = rk[rk_offset + 6];
        t3 = rk[rk_offset + 7];

        t0 = aes_enc_round(t0, s0, AesByte::Byte0);
        t1 = aes_enc_round(t1, s1, AesByte::Byte0);
        t2 = aes_enc_round(t2, s2, AesByte::Byte0);
        t3 = aes_enc_round(t3, s3, AesByte::Byte0);

        t0 = aes_enc_round(t0, s1, AesByte::Byte1);
        t1 = aes_enc_round(t1, s2, AesByte::Byte1);
        t2 = aes_enc_round(t2, s3, AesByte::Byte1);
        t3 = aes_enc_round(t3, s0, AesByte::Byte1);

        t0 = aes_enc_round(t0, s2, AesByte::Byte2);
        t1 = aes_enc_round(t1, s3, AesByte::Byte2);
        t2 = aes_enc_round(t2, s0, AesByte::Byte2);
        t3 = aes_enc_round(t3, s1, AesByte::Byte2);

        t0 = aes_enc_round(t0, s3, AesByte::Byte3);
        t1 = aes_enc_round(t1, s0, AesByte::Byte3);
        t2 = aes_enc_round(t2, s1, AesByte::Byte3);
        t3 = aes_enc_round(t3, s2, AesByte::Byte3);

        rk_offset += 8;
        round_count -= 1;
        if round_count == 0 {
            break;
        }

        s0 = rk[rk_offset + 0];
        s1 = rk[rk_offset + 1];
        s2 = rk[rk_offset + 2];
        s3 = rk[rk_offset + 3];

        s0 = aes_enc_round(s0, t0, AesByte::Byte0);
        s1 = aes_enc_round(s1, t1, AesByte::Byte0);
        s2 = aes_enc_round(s2, t2, AesByte::Byte0);
        s3 = aes_enc_round(s3, t3, AesByte::Byte0);

        s0 = aes_enc_round(s0, t1, AesByte::Byte1);
        s1 = aes_enc_round(s1, t2, AesByte::Byte1);
        s2 = aes_enc_round(s2, t3, AesByte::Byte1);
        s3 = aes_enc_round(s3, t0, AesByte::Byte1);

        s0 = aes_enc_round(s0, t2, AesByte::Byte2);
        s1 = aes_enc_round(s1, t3, AesByte::Byte2);
        s2 = aes_enc_round(s2, t0, AesByte::Byte2);
        s3 = aes_enc_round(s3, t1, AesByte::Byte2);

        s0 = aes_enc_round(s0, t3, AesByte::Byte3);
        s1 = aes_enc_round(s1, t0, AesByte::Byte3);
        s2 = aes_enc_round(s2, t1, AesByte::Byte3);
        s3 = aes_enc_round(s3, t2, AesByte::Byte3);
    }

    s0 = rk[rk_offset + 0];
    s1 = rk[rk_offset + 1];
    s2 = rk[rk_offset + 2];
    s3 = rk[rk_offset + 3];

    s0 = aes_enc_round_last(s0, t0, AesByte::Byte0);
    s1 = aes_enc_round_last(s1, t1, AesByte::Byte0);
    s2 = aes_enc_round_last(s2, t2, AesByte::Byte0);
    s3 = aes_enc_round_last(s3, t3, AesByte::Byte0);

    s0 = aes_enc_round_last(s0, t1, AesByte::Byte1);
    s1 = aes_enc_round_last(s1, t2, AesByte::Byte1);
    s2 = aes_enc_round_last(s2, t3, AesByte::Byte1);
    s3 = aes_enc_round_last(s3, t0, AesByte::Byte1);

    s0 = aes_enc_round_last(s0, t2, AesByte::Byte2);
    s1 = aes_enc_round_last(s1, t3, AesByte::Byte2);
    s2 = aes_enc_round_last(s2, t0, AesByte::Byte2);
    s3 = aes_enc_round_last(s3, t1, AesByte::Byte2);

    s0 = aes_enc_round_last(s0, t3, AesByte::Byte3);
    s1 = aes_enc_round_last(s1, t0, AesByte::Byte3);
    s2 = aes_enc_round_last(s2, t1, AesByte::Byte3);
    s3 = aes_enc_round_last(s3, t2, AesByte::Byte3);

    set_u32(output, 0, s0);
    set_u32(output, 4, s1);
    set_u32(output, 8, s2);
    set_u32(output, 12, s3);
}

pub fn vexriscv_aes_decrypt(input: &[u8], output: &mut [u8], key: &Aes) {
    let rk = key.key;

    // We do two rounds per loop
    let mut round_count = key.rounds / 2;

    let mut s0 = get_u32_le(input, 0);
    let mut s1 = get_u32_le(input, 4);
    let mut s2 = get_u32_le(input, 8);
    let mut s3 = get_u32_le(input, 12);

    let mut t0 = rk[0];
    let mut t1 = rk[1];
    let mut t2 = rk[2];
    let mut t3 = rk[3];

    s0 ^= t0;
    s1 ^= t1;
    s2 ^= t2;
    s3 ^= t3;

    let mut rk_offset = 0;
    loop {
        t0 = rk[rk_offset + 4];
        t1 = rk[rk_offset + 5];
        t2 = rk[rk_offset + 6];
        t3 = rk[rk_offset + 7];

        t0 = aes_dec_round(t0, s0, AesByte::Byte0);
        t1 = aes_dec_round(t1, s1, AesByte::Byte0);
        t2 = aes_dec_round(t2, s2, AesByte::Byte0);
        t3 = aes_dec_round(t3, s3, AesByte::Byte0);

        t0 = aes_dec_round(t0, s3, AesByte::Byte1);
        t1 = aes_dec_round(t1, s0, AesByte::Byte1);
        t2 = aes_dec_round(t2, s1, AesByte::Byte1);
        t3 = aes_dec_round(t3, s2, AesByte::Byte1);

        t0 = aes_dec_round(t0, s2, AesByte::Byte2);
        t1 = aes_dec_round(t1, s3, AesByte::Byte2);
        t2 = aes_dec_round(t2, s0, AesByte::Byte2);
        t3 = aes_dec_round(t3, s1, AesByte::Byte2);

        t0 = aes_dec_round(t0, s1, AesByte::Byte3);
        t1 = aes_dec_round(t1, s2, AesByte::Byte3);
        t2 = aes_dec_round(t2, s3, AesByte::Byte3);
        t3 = aes_dec_round(t3, s0, AesByte::Byte3);

        rk_offset += 8;
        round_count -= 1;
        if round_count == 0 {
            break;
        }

        s0 = rk[rk_offset + 0];
        s1 = rk[rk_offset + 1];
        s2 = rk[rk_offset + 2];
        s3 = rk[rk_offset + 3];

        s0 = aes_dec_round(s0, t0, AesByte::Byte0);
        s1 = aes_dec_round(s1, t1, AesByte::Byte0);
        s2 = aes_dec_round(s2, t2, AesByte::Byte0);
        s3 = aes_dec_round(s3, t3, AesByte::Byte0);

        s0 = aes_dec_round(s0, t3, AesByte::Byte1);
        s1 = aes_dec_round(s1, t0, AesByte::Byte1);
        s2 = aes_dec_round(s2, t1, AesByte::Byte1);
        s3 = aes_dec_round(s3, t2, AesByte::Byte1);

        s0 = aes_dec_round(s0, t2, AesByte::Byte2);
        s1 = aes_dec_round(s1, t3, AesByte::Byte2);
        s2 = aes_dec_round(s2, t0, AesByte::Byte2);
        s3 = aes_dec_round(s3, t1, AesByte::Byte2);

        s0 = aes_dec_round(s0, t1, AesByte::Byte3);
        s1 = aes_dec_round(s1, t2, AesByte::Byte3);
        s2 = aes_dec_round(s2, t3, AesByte::Byte3);
        s3 = aes_dec_round(s3, t0, AesByte::Byte3);
    }

    s0 = rk[rk_offset + 0];
    s1 = rk[rk_offset + 1];
    s2 = rk[rk_offset + 2];
    s3 = rk[rk_offset + 3];

    s0 = aes_dec_round_last(s0, t0, AesByte::Byte0);
    s1 = aes_dec_round_last(s1, t1, AesByte::Byte0);
    s2 = aes_dec_round_last(s2, t2, AesByte::Byte0);
    s3 = aes_dec_round_last(s3, t3, AesByte::Byte0);

    s0 = aes_dec_round_last(s0, t3, AesByte::Byte1);
    s1 = aes_dec_round_last(s1, t0, AesByte::Byte1);
    s2 = aes_dec_round_last(s2, t1, AesByte::Byte1);
    s3 = aes_dec_round_last(s3, t2, AesByte::Byte1);

    s0 = aes_dec_round_last(s0, t2, AesByte::Byte2);
    s1 = aes_dec_round_last(s1, t3, AesByte::Byte2);
    s2 = aes_dec_round_last(s2, t0, AesByte::Byte2);
    s3 = aes_dec_round_last(s3, t1, AesByte::Byte2);

    s0 = aes_dec_round_last(s0, t1, AesByte::Byte3);
    s1 = aes_dec_round_last(s1, t2, AesByte::Byte3);
    s2 = aes_dec_round_last(s2, t3, AesByte::Byte3);
    s3 = aes_dec_round_last(s3, t0, AesByte::Byte3);

    set_u32(output, 0, s0);
    set_u32(output, 4, s1);
    set_u32(output, 8, s2);
    set_u32(output, 12, s3);
}
