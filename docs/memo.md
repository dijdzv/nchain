### web url
http://192.168.1.17/

### set permission for bind port 80
```bash
sudo setcap CAP_NET_BIND_SERVICE=+eip /home/dijdzv/github/nchain/target/release/nchain
```