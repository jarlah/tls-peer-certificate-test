rust-native-tls is not able to receive a peer certificate.

To reproduce the problem:

1. Run curl
```
curl https://127.0.0.1:8443 --cacert ./certs/ca.pem --cert ./certs/CPE1234/cert.pem --key ./certs/CPE1234/key.pem --cert-type PEM -v --http0.9 -d 'hello native-tls'
```
2. Verify that peer certificate is not received by server
```
0x0000000143904580 <- and what is this random number logged for each request?
Peer certificate: None <- should in this example be the debugged ouput of an Option of Vec<u8>
```