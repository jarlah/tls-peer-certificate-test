rust-native-tls is not able to receive a peer certificate.

Operating system:
macOS Ventura 13.3.1

OpenSSL version:
LibreSSL 3.3.6

To reproduce the problem:

1. cargo run
2. Run curl
    ```
    curl https://127.0.0.1:8443 --cacert ./certs/ca.pem --cert ./certs/peer/cert.pem --key ./certs/peer/key.pem --cert-type PEM -v --http0.9 -d 'hello native-tls'
    ```
3. Verify that peer certificate is not received by server
    ```
    0x0000000143904580 <- and what is this random number logged for each request?
    Peer certificate: None <- should in this example be the debugged ouput of an Option of Vec<u8>
    ```

ANY CERTIFICATE IN THIS REPOSITORY IS DUMMY CERTIFICATES