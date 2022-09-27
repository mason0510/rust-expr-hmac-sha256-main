/// Test that hmac_sha256 produce the expected results
/// as defined by [binance.us api docs](https://github.com/binance-us/binance-official-api-docs).

use hmac_sha256::HMAC;

#[macro_use]
extern crate hex_literal;


fn main() -> () {
    // Data, key and expected
    // from: https://github.com/binance-us/binance-official-api-docs/blob/fc916164ae04eb2e95ff7f98c2d49d8d6bd6d096/rest-api.md#example-2-as-a-query-string
    let data = b"symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
    let key = b"NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
    let expected = hex!("c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71");

    // Calculate the signature from the data and key
    let signature = HMAC::mac(data, key);
    println!("signature ={:02x?}", signature);
    // Compare the signature to the expected
    //转成字符串类型
    // let signature_str = hex::encode(signature);
    //打印

    // Validate
    assert_eq!(signature, expected);

    //is or not equal
    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);
    // assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

}
