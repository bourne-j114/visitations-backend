// extern crate rand;

use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use binascii::{hex2bin, bin2hex};
use std::str;
use rand::prelude::*;
use rand::rngs::OsRng;

// Encrypt a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
#[allow(dead_code)]
pub fn encrypt(data: String, key: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {

    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::default();
    rng.fill_bytes(&mut iv);

    // Create an encryptor instance of the best performing
    // type available for the platform.
    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize256,
        key,
        iv.as_ref(),
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    let iv_buffer = &mut [0u8; 16];
    let iv_hex = bin2hex(&iv, iv_buffer).ok().unwrap();

    let data_buffer = &mut [0u8; 16];
    let data_hex = bin2hex(&final_result, data_buffer).ok().unwrap();
    let iv_str = str::from_utf8(iv_hex).unwrap();
    let data_str = str::from_utf8(data_hex).unwrap();

    let encrypted_data = format!("{}:{}", iv_str, data_str);

    Ok(encrypted_data)
}

// Decrypts a buffer with the given key and iv using
// AES-256/CBC/Pkcs encryption.
#[allow(dead_code)]
pub fn decrypt(data: String, key: &[u8]) -> Result<String, symmetriccipher::SymmetricCipherError> {
    let encrypted: Vec<&str> = data.split(":").collect();

    let vi_buffer = &mut [0u8; 32];
    let iv = hex2bin(encrypted[0].as_ref(), vi_buffer).ok().unwrap();
    // println!("IV: {:?}", &iv);

    let data_buffer = &mut [0u8; 32];
    let enc_data = hex2bin(encrypted[1].as_ref(), data_buffer).ok().unwrap();
    // println!("ENC DATA: {:?}", &enc_data);

    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize256,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(enc_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    let decrypted = str::from_utf8(&final_result).unwrap();

    Ok(decrypted.to_string())
}