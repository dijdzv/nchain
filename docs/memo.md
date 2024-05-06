### set permission for bind port 80
```bash
sudo setcap CAP_NET_BIND_SERVICE=+eip /home/dijdzv/github/nchain/target/release/nchain
```