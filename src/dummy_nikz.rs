use crate::traits::NIZKProof;
use rand::prelude::ThreadRng;

pub struct DummyNIZK();
#[derive(Debug, Clone)]
pub struct DummyCRS();
#[derive(Debug, Clone)]
pub struct DummyProof();
#[derive(Debug, Clone)]
pub struct DummyWord();
#[derive(Debug, Clone)]
pub struct DummyWitness();

impl NIZKProof<DummyCRS, DummyWord, DummyWitness, DummyProof> for DummyNIZK {
    fn crs_generation(_: usize, _: &mut ThreadRng) -> DummyCRS {
        panic!("you dummy");
    }
    fn prove(_: &DummyCRS, _: &DummyWord, _: &DummyWitness, _: &mut ThreadRng) -> DummyProof {
        panic!("you dummy");
    }
    fn verify(_: &DummyCRS, _: &DummyWord, _: &DummyProof, _: &mut ThreadRng) -> bool {
        panic!("you dummy");
    }
}
