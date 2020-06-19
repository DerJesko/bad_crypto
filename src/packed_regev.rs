use crate::distributions::{BoundedFiniteGauss, SmallFlat};
use ndarray::{arr1, Array1, Array2, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn round_between_modulus(x: usize, input_modulus: usize, output_modulus: usize) -> usize {
    ((((x * output_modulus) as f64) / (input_modulus as f64)).round() as usize) % output_modulus
}

pub struct BatchedRegev {
    lattice_row: usize,
    lattice_column: usize,
    lattice_modulus: usize,
    distribution_bound: usize,
    num_messages: usize,
    message_modulus: usize,
}

#[derive(Clone, Debug)]
pub struct PublicKey {
    lattice_base: Array2<usize>,
    approx_secret_subspaces: Array2<usize>,
}

#[derive(Debug)]
pub struct Ciphertext(Array2<usize>, Array2<usize>);

#[derive(Debug)]
pub struct ShrunkCiphertext(Array2<usize>, usize, Array2<usize>);

#[derive(PartialEq, Debug)]
pub struct Message(Array1<usize>);

pub struct SecretKey(Array2<usize>, PublicKey);

impl BatchedRegev {
    pub fn new(sec_param: usize, functionality_param: usize) -> Self {
        BatchedRegev {
            lattice_row: 6,
            lattice_column: 4,
            lattice_modulus: sec_param,
            distribution_bound: 5,
            num_messages: 5,
            message_modulus: 3,
        }
    }
}

impl BatchedRegev {
    pub fn key_generation(&self) -> (PublicKey, SecretKey) {
        let lattice_base = Array2::random(
            (self.lattice_row, self.lattice_column),
            Uniform::new(0, self.lattice_modulus),
        );
        let secret = Array2::random(
            (self.lattice_column, self.num_messages),
            Uniform::new(0, self.lattice_modulus),
        );
        let error = Array2::random(
            (self.lattice_row, self.num_messages),
            BoundedFiniteGauss::new(0, self.lattice_modulus),
        );
        let pk = PublicKey {
            approx_secret_subspaces: (lattice_base.dot(&secret) + error) % self.lattice_modulus,
            lattice_base,
        };
        (pk.clone(), SecretKey(secret, pk))
    }

    pub fn encrypt(&self, pub_key: &PublicKey, message: &Message) -> Ciphertext {
        let Message(m) = message;
        let randomizer = Array2::random(
            (1, self.lattice_row),
            SmallFlat::new(1, self.lattice_modulus),
        );
        let c1 = randomizer.dot(&pub_key.lattice_base) % self.lattice_modulus;
        let c2 = (randomizer.dot(&pub_key.approx_secret_subspaces)
            + (m * self.lattice_modulus / self.message_modulus))
            % self.lattice_modulus;
        Ciphertext(c1, c2)
    }

    pub fn shrink_ciphertext(&self, cipher_text: &Ciphertext) -> ShrunkCiphertext {
        let mut forbidden_zone = Vec::new();
        for c2 in cipher_text.1.iter() {
            for i in 0..self.message_modulus {
                let forbidden_value = ((self.lattice_modulus * 2 * i) / (2 * self.message_modulus))
                    + self.lattice_modulus
                    - c2;
                forbidden_zone.push((
                    (forbidden_value - self.distribution_bound) % self.lattice_modulus,
                    (forbidden_value + self.distribution_bound) % self.lattice_modulus,
                ))
            }
        }
        forbidden_zone.sort();
        let mut r = forbidden_zone[0].1 + 1;
        for (start, end) in forbidden_zone {
            if r < start {
                break;
            }
            r = end + 1;
        }
        ShrunkCiphertext(
            cipher_text.0.clone(),
            r,
            cipher_text
                .1
                .mapv(|x| round_between_modulus(x, self.lattice_modulus, self.message_modulus)),
        )
    }

    pub fn unshrink_ciphertext(&self, shrunk_cipher_text: &ShrunkCiphertext) -> Ciphertext {
        Ciphertext(
            shrunk_cipher_text.0.clone(),
            shrunk_cipher_text.2.mapv(|x| {
                (round_between_modulus(x, self.message_modulus, self.lattice_modulus)
                    + shrunk_cipher_text.1)
                    % self.lattice_modulus
            }),
        )
    }

    pub fn decrypt(&self, sec_key: &SecretKey, cipher_text: &Ciphertext) -> Option<Message> {
        let Ciphertext(c1, c2) = cipher_text;
        let SecretKey(secret, _) = sec_key;
        let linear_decryption =
            ((Array2::<usize>::zeros((1, self.num_messages)) + self.lattice_modulus) + c2.clone()
                - (c1.dot(secret) % self.lattice_modulus))
                % self.lattice_modulus;
        let decryption = (linear_decryption.index_axis_move(Axis(0), 0))
            .mapv(|x| round_between_modulus(x, self.lattice_modulus, self.message_modulus));
        Some(Message(decryption))
    }
}

#[test]
fn test_basic_functions() {
    let reg = BatchedRegev::new(128, 1);
    let (pk, sk) = reg.key_generation();
    for i in 0..3 {
        let message = Message(arr1(&[i; 5]));
        let cipher_text = reg.encrypt(&pk, &message);
        let decrypted_value = reg.decrypt(&sk, &cipher_text).unwrap();
        assert_eq!(message, decrypted_value)
    }
}

#[test]
fn test_shrink() {
    let reg = BatchedRegev::new(128, 1);
    let (pk, sk) = reg.key_generation();
    for i in 0..3 {
        let message = Message(arr1(&[i; 5]));
        let cipher_text = reg.encrypt(&pk, &message);
        let shrunk_cipher_text = reg.shrink_ciphertext(&cipher_text);
        let unshrunk_cipher_text = reg.unshrink_ciphertext(&shrunk_cipher_text);
        println!("c1:{:?}", cipher_text.1);
        println!("c2:{:?}", unshrunk_cipher_text.1);
        let decrypted_value = reg.decrypt(&sk, &unshrunk_cipher_text).unwrap();
        assert_eq!(message, decrypted_value)
    }
}
