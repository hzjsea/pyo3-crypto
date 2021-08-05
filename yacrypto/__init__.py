from yacrypto import Crypto,generate_key
import uuid


def echo():
    pub_key, pri_key = generate_key(1024)
    
    a = Crypto(pub_key,pri_key)
    data = "helloworld"
    


    print("data => {}",data)
    data1 = a.public_encrypt(data)
    print("pubkey encrypt => {}", a.public_encrypt(data))
    print("prikey decrypt => {}", a.private_decrypt(data1))

    print("data => {}",data)
    data2 = a.private_encrypt(data)
    print("prikey encrypt => {}", a.private_encrypt(data))
    print("pubkey encrypt => {}", a.public_decrypt(data2))



if __name__ == "__main__":
    echo()
    