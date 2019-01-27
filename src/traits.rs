use rand::prelude::ThreadRng;

pub trait PubKEncryption<PubK, SecK, Mes, Ciph> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PubK, SecK);
    fn encrypt(pub_key: &PubK, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(sec_key: &SecK, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}

pub trait HomomorphEncryption<PubK, SecK, Mes, Ciph, Func> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PubK, SecK);
    fn encrypt(pub_key: &PubK, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn eval(pub_key: &PubK, function: Func, ciphertexts: Vec<Ciph>, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(sec_key: &SecK, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}

pub trait TagEncryption<PubK, SecK, Mes, Ciph, Tag> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PubK, SecK);
    fn encrypt(pub_key: &PubK, tag: &Tag, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(sec_key: &SecK, tag: &Tag, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}

pub trait PrivKEncryption<K, Mes, Ciph> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> K;
    fn encrypt(key: &K, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(key: &K, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}

pub trait NIZKProof<CRS, X, W, Proof> {
    fn crs_generation(sec_param: usize, rng: &mut ThreadRng) -> CRS;
    fn prove(crs: &CRS, x: &X, w: &W, rng: &mut ThreadRng) -> Proof;
    fn verify(crs: &CRS, x: &X, proof: &Proof, rng: &mut ThreadRng) -> bool;
}

pub trait BilinerMap<G1, G2> {
    fn bilinear_map(g: G1, h: G1) -> G2;
}
