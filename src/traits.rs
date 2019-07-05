use rand::prelude::ThreadRng;

pub trait PubKEncryption<PubK, SecK, Mes, Ciph> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> (PubK, SecK);
    fn encrypt(pub_key: &PubK, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(sec_key: &SecK, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}

pub trait HomomorphEncryption<PubK, SecK, Mes, Ciph, Func>:
    PubKEncryption<PubK, SecK, Mes, Ciph>
{
    fn eval(pub_key: &PubK, function: Func, ciphertexts: Vec<Ciph>, rng: &mut ThreadRng) -> Ciph;
}

pub trait PrivKEncryption<K, Mes, Ciph> {
    fn key_generation(sec_param: usize, rng: &mut ThreadRng) -> K;
    fn encrypt(key: &K, message: &Mes, rng: &mut ThreadRng) -> Ciph;
    fn decrypt(key: &K, cipher_text: &Ciph, rng: &mut ThreadRng) -> Option<Mes>;
}
