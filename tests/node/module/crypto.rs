use node_sys::crypto;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn constants() {
    let constants = &crypto::constants;
    let _ = constants.ALPN_ENABLED();
    let _ = constants.DH_CHECK_P_NOT_PRIME();
    let _ = constants.DH_CHECK_P_NOT_SAFE_PRIME();
    let _ = constants.DH_NOT_SUITABLE_GENERATOR();
    let _ = constants.DH_UNABLE_TO_CHECK_GENERATOR();
    let _ = constants.ENGINE_METHOD_ALL();
    let _ = constants.ENGINE_METHOD_CIPHERS();
    let _ = constants.ENGINE_METHOD_DH();
    let _ = constants.ENGINE_METHOD_DIGESTS();
    let _ = constants.ENGINE_METHOD_DSA();
    let _ = constants.ENGINE_METHOD_EC();
    let _ = constants.ENGINE_METHOD_NONE();
    let _ = constants.ENGINE_METHOD_PKEY_ASN1_METHS();
    let _ = constants.ENGINE_METHOD_PKEY_METHS();
    let _ = constants.ENGINE_METHOD_RAND();
    let _ = constants.ENGINE_METHOD_RSA();
    let _ = constants.OPENSSL_VERSION_NUMBER();
    let _ = constants.RSA_NO_PADDING();
    let _ = constants.RSA_PKCS1_OAEP_PADDING();
    let _ = constants.RSA_PKCS1_PADDING();
    let _ = constants.RSA_PKCS1_PSS_PADDING();
    let _ = constants.RSA_PSS_SALTLEN_AUTO();
    let _ = constants.RSA_PSS_SALTLEN_DIGEST();
    let _ = constants.RSA_PSS_SALTLEN_MAX_SIGN();
    let _ = constants.RSA_SSLV23_PADDING();
    let _ = constants.RSA_X931_PADDING();
    let _ = constants.SSL_OP_ALL();
    let _ = constants.SSL_OP_ALLOW_UNSAFE_LEGACY_RENEGOTIATION();
    let _ = constants.SSL_OP_CIPHER_SERVER_PREFERENCE();
    let _ = constants.SSL_OP_CISCO_ANYCONNECT();
    let _ = constants.SSL_OP_COOKIE_EXCHANGE();
    let _ = constants.SSL_OP_CRYPTOPRO_TLSEXT_BUG();
    let _ = constants.SSL_OP_DONT_INSERT_EMPTY_FRAGMENTS();
    let _ = constants.SSL_OP_EPHEMERAL_RSA();
    let _ = constants.SSL_OP_LEGACY_SERVER_CONNECT();
    let _ = constants.SSL_OP_MICROSOFT_BIG_SSLV3_BUFFER();
    let _ = constants.SSL_OP_MICROSOFT_SESS_ID_BUG();
    let _ = constants.SSL_OP_MSIE_SSLV2_RSA_PADDING();
    let _ = constants.SSL_OP_NETSCAPE_CA_DN_BUG();
    let _ = constants.SSL_OP_NETSCAPE_CHALLENGE_BUG();
    let _ = constants.SSL_OP_NETSCAPE_DEMO_CIPHER_CHANGE_BUG();
    let _ = constants.SSL_OP_NETSCAPE_REUSE_CIPHER_CHANGE_BUG();
    let _ = constants.SSL_OP_NO_COMPRESSION();
    let _ = constants.SSL_OP_NO_QUERY_MTU();
    let _ = constants.SSL_OP_NO_SESSION_RESUMPTION_ON_RENEGOTIATION();
    let _ = constants.SSL_OP_NO_SSLv2();
    let _ = constants.SSL_OP_NO_SSLv3();
    let _ = constants.SSL_OP_NO_TICKET();
    let _ = constants.SSL_OP_NO_TLSv1_1();
    let _ = constants.SSL_OP_NO_TLSv1_2();
    let _ = constants.SSL_OP_NO_TLSv1();
    let _ = constants.SSL_OP_PKCS1_CHECK_1();
    let _ = constants.SSL_OP_PKCS1_CHECK_2();
    let _ = constants.SSL_OP_SINGLE_DH_USE();
    let _ = constants.SSL_OP_SINGLE_ECDH_USE();
    let _ = constants.SSL_OP_SSLEAY_080_CLIENT_DH_BUG();
    let _ = constants.SSL_OP_SSLREF2_REUSE_CERT_TYPE_BUG();
    let _ = constants.SSL_OP_TLS_BLOCK_PADDING_BUG();
    let _ = constants.SSL_OP_TLS_D5_BUG();
    let _ = constants.SSL_OP_TLS_ROLLBACK_BUG();
}

#[wasm_bindgen_test]
pub fn create_hash() {
    let algorithm = &"md5";
    let options = Default::default();
    crypto::create_hash(algorithm, options);
}
