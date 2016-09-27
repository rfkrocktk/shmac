/**
 * libsodium KDF bindings.
 */

use libc::{c_int, size_t};


#[repr(C)]
pub enum Argon2ErrorCodes {
    OK                      = 0,
    OutputPointerNull       = -1,
    OutputTooShort          = -2,
    OutputTooLong           = -3,
    PasswordTooShort        = -4,
    PasswordTooLong         = -5,
    SaltTooShort            = -6,
    SaltTooLong             = -7,
    ADTooShort              = -8,
    ADTooLong               = -9,
    SecretTooShort          = -10,
    SecretTooLong           = -11,
    TooLittleTime           = -12,
    TooMuchTime             = -13,
    TooLittleMemory         = -14,
    TooMuchMemory           = -15,
    TooFewLanes             = -16,
    TooManyLanes            = -17,
    PasswordPointerMismatch = -18,
    SaltPointerMismatch     = -19,
    SecretPointerMismatch   = -20,
    ADPointerMismatch       = -21,
    MemoryAlocationError    = -22,
    FreeMemoryCBKNull       = -23,
    AllocateMemoryCBKNull   = -24,
    IncorrectParameter      = -25,
    IncorrectType           = -26,
    OutputPointerMismatch   = -27,
    TooFewThreads           = -28,
    TooManyThreads          = -29,
    MissingArgs             = -30,
    EncodingFail            = -31,
    DecodingFail            = -32,
    ThreadFail              = -33,
    DecodingLengthFail      = -34,
    VerifyMismatch          = -35,
}

#[link(name = "sodium")]
extern {
    /*
    int argon2i_hash_raw(const uint32_t t_cost, const uint32_t m_cost,
                     const uint32_t parallelism, const void *pwd,
                     const size_t pwdlen, const void *salt,
                     const size_t saltlen, void *hash, const size_t hashlen);
    */
    pub fn argon2i_hash_raw(
        t_cost: u32,
        m_cost: u32,
        parallelism: u32,
        pwd: *const u8,
        pwdlen: size_t,
        salt: *const u8,
        saltlen: size_t,
        hash: *mut [u8],
        hashlen: size_t,
    ) -> c_int;
}

pub fn safe_argon2i_hash_raw_32(t_cost: u32, m_cost: u32, parallelism: u32, pwd: &[u8],
        salt: &[u8]) -> Result<[u8; 32], i32> {

    let mut output = [0; 32];

    let rc = unsafe {
        argon2i_hash_raw(t_cost, m_cost, parallelism, pwd.as_ptr(), pwd.len() as size_t,
            salt.as_ptr(), salt.len() as size_t, &mut output, 32)
    };

    match rc {
        0 => Ok(output),
        _ => Err(rc),
    }
}
