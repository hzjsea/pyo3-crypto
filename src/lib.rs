use std::u32;
use pyo3::prelude::*;
use openssl::rsa::{Padding,Rsa};

const SECRET: &'static str = "CHFfxQA3tqEZgKusgwZjmI5lFsoZxXGXnQLA97oYga2M33sLwREZyy1mWCM8GIIA";

mod crypto_utils {
    use hmac::{Hmac, Mac, NewMac};
    use sha2::Sha256;
    use std::fmt::Write;

    type Hmacsha256 = Hmac<Sha256>;
    fn encode_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            match write!(&mut s, "{:02x}", b) {
                Ok(_) => {},
                Err(_) => {}
            };
        }
        s
    }
    
    pub fn hash_hmac(secret: &str, msg: &str) -> String {
        let mut mac = Hmacsha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
        mac.update(msg.as_bytes());
        let result = mac.finalize();
        let code_bytes = result.into_bytes();
        encode_hex(&code_bytes)
    }

}

// fn openssl_public_decrypt(pk:&[u8], msg: &[u8]) -> Result<Vec<u8>, ()> {
//     let mut out: [u8; 4096] = [0;4096];
//     let rsa = Rsa::public_key_from_pem(pk).unwrap();
//     let size = rsa.public_decrypt(msg, &mut out, Padding::PKCS1).unwrap();
//     Ok(out[..size].to_vec())
// }


// fn openssl_private_encrypt(pk:&[u8], msg: &[u8]) -> Result<Vec<u8>,()> {
//     let mut out: [u8; 4096] = [0;4096];
//     let rsa = Rsa::private_key_from_pem(pk).unwrap();
//     let size = rsa.private_encrypt(msg, &mut out, Padding::PKCS1).unwrap();
//     Ok(out[..size].to_vec())
// }


// create public/private key  create_key(1024)
fn create_key(len:u32) -> (String,String){
    let rsa = openssl::rsa::Rsa::generate(len).unwrap();
    let pubkey = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();
    let prikey  = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();

    (pubkey, prikey)
}



#[pyclass]
struct Crypto {
    // #[pyo3(get, set)]
    // pubkey: String,
    // #[pyo3(get,set)]
    // prikey: String,
    pub_key: Rsa<openssl::pkey::Public>,
    pri_key: Rsa<openssl::pkey::Private>
}

#[pyfunction]
fn generate_key(len:u32) -> (String, String){
    create_key(len)
}

#[pymethods]
impl Crypto {
    #[new]
    pub fn __new__(pubkey: &str,prikey: &str) -> Self {
        Crypto {
            // pubkey: pubkey.to_owned(),
            // prikey: prikey.to_owned(),
            pub_key: Rsa::public_key_from_pem(pubkey.as_bytes()).unwrap(),
            pri_key: Rsa::private_key_from_pem(prikey.as_bytes()).unwrap(),
        }
    }

    // public decrypt 
    pub fn public_decrypt(&self, msg:&str) -> String {
        let mut out: [u8; 4096] = [0;4096];
        let decoded = openssl::base64::decode_block(msg).unwrap();
        if let Ok(size) = self.pub_key.public_decrypt(&decoded, &mut out, Padding::PKCS1) {
            let real_size = if size > 4096 {4096} else {size};
            // openssl::base64::encode_block(&out[..real_size])
            String::from_utf8(out[..real_size].to_vec()).unwrap()
        } else {
            String::default()
        }
    }
    
    // public encrypt 
    pub fn public_encrypt(&self, msg:&str) -> String {
        let mut out: [u8; 4096] = [0;4096];
        if let Ok(size) = self.pub_key.public_encrypt(msg.as_bytes(), &mut out, Padding::PKCS1) {
            let real_size = if size > 4096 {4096}else{size};
            openssl::base64::encode_block(&out[..real_size])
        } else {
            String::default()
        }
    }

    // private encrypt
    pub fn private_encrypt(&self, msg:&str) -> String{
        let mut out: [u8; 4096] = [0;4096];
        if let Ok(size) = self.pri_key.private_encrypt(msg.as_bytes(), &mut out, Padding::PKCS1) {
            let real_size = if size > 4096 {4096}else{size};
            openssl::base64::encode_block(&out[..real_size])
        } else {
            String::default()
        }
    }

    // private decrypt
    pub fn private_decrypt(&self, msg: &str) -> String{
        let mut out: [u8; 4096] = [0;4096];
        let decoded = openssl::base64::decode_block(msg).unwrap();
        if let Ok(size) = self.pri_key.private_decrypt(&decoded, &mut out, Padding::PKCS1) {
            let real_size = if size > 4096 {4096} else {size};
            // openssl::base64::encode_block(&out[..real_size])
            String::from_utf8(out[..real_size].to_vec()).unwrap()
        } else {
            String::default()
        } 
    }

    // sign
    pub fn sign(&self, msg: &str) ->String {
        crypto_utils::hash_hmac(SECRET, msg)
    }
}

#[pymodule]
fn yacrypto(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Crypto>()?;
    m.add_function(wrap_pyfunction!(generate_key, m)?).unwrap();
    Ok(())
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // let key = "-----BEGIN PUBLIC KEY-----\nMIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCq1HmEIEUJbyRB4uX+sXcaRF83\n5lv8WxQGo/W2TL1dvYdKQxqNNdgmW9M5+tnxQ5O+Y5uzkTW3voYgmPRFj8/BLqZu\nygQDeOrn3Wan69b4FewwUanJugA36wyGHS9QkjMTO1aabkXiuEeWreF8jel0nPSG\nejKtqo3ZASqldL8nsQIDAQAB\n-----END PUBLIC KEY-----\n";
        // let msg = "b4LIUV8m+Yl880NDMhQ9PYdZc8VlYib7KgTPrdMX8/Hjvmr371z2+x1KlerNR5CAG1uY0Qq2JLrrQOqteNWkt6kHxGE48TG3YfgjOxoYf3yeEcNtpI9EuoLJ8NyGoXtdHjHR5r/Ghlb02ZiVpGExxREmW5cq2ZbXpPvJoMAYtdU=";
        // let data = openssl::base64::decode_block(&msg).unwrap(); // string to bytes
        // // decode
        // // println!("data =>  {:?}",data);
        // let r = super::openssl_public_decrypt(key.as_bytes(),&data).unwrap();
        // println!("r => {:?}",r);
        // let fdata = openssl::base64::encode_block(&r); // bytes to string
        // let orgin = "hPSHp6gtv00BpKmRtbZNrPSNMB25u2xU";

        // println!("fdata => {:?}", fdata);
        // println!("origin: {}", openssl::base64::encode_block(orgin.as_bytes()));
        // assert_eq!(openssl::base64::encode_block(orgin.as_bytes()),fdata);
    }
    
    
    use base64;
    #[test]
    fn works(){

        // create rsa
        let rsa = openssl::rsa::Rsa::generate(1024).unwrap();
        // create public key 
        let public_key = rsa.public_key_to_pem().unwrap();
        println!("{:?}", String::from_utf8(public_key.clone()));
        let private_key = rsa.private_key_to_pem().unwrap();


        let data = "hellowo\n\t\rrld";
        // public encrypt 
        let mut buf:Vec<u8> = vec![0;rsa.size() as usize];
        let rsa_pub = openssl::rsa::Rsa::public_key_from_pem(&public_key).unwrap();
        let _ = rsa_pub.public_encrypt(data.as_bytes(), &mut buf , openssl::rsa::Padding::PKCS1);

        // private decrypt => 
        let data = buf;
        let mut buf:Vec<u8> = vec![0;rsa.size() as usize];
        let rsa_pri  = openssl::rsa::Rsa::private_key_from_pem(&private_key).unwrap();
        if let Ok(size) = rsa_pri.private_decrypt(&data, &mut buf, openssl::rsa::Padding::PKCS1){
            let real_size = if size > 1024 {1024} else {size};
            let buf = &buf[..real_size];
            let base64_string = openssl::base64::encode_block(&buf);
            let result = base64::decode(base64_string);
            println!("{:?}",result);
            // println!("buf => {:?}",openssl::base64::encode_block(&buf))

            let echo_str = String::from_utf8((&buf).to_vec()).unwrap();
            println!("{:?}",echo_str);
        
        }
    }
}
