#[derive(Debug, Clone, Copy)]
pub enum CrustData {
    BBP1971,    // BaymBethePethick1971,
    BPS1971,    // BaymPethickSutherland1971,
    AccFeNPC,   // AccFenopycHZ2008,
    AccFePCN,   // AccFepycnoHZ2008,
    AccPdNPC,   // AccPdnopycHZ2008,
    AccPdPCN,   // AccPdpycnoHZ2008,
    CatHZDNV,   // CatHZDNV,
    HZ1990,     // HaenselZdunik1990,
    NV1973,     // NegeleVautherin1973,
    NV19731,    // NegeleVautherin1973EOS1,
    OCFeEOS,    // OuterCrustFeEOS,
    OCPdEOS     // OuterCrustPdEOS,
}