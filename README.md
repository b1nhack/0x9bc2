# 0x9bc2
Customized network layer protocol C2

blog post: https://b1n.io/posts/customized-network-layer-protocol-c2

`0x9b`is the protocol number I used  
This repo just a PoC for Customized network layer protocol C2  
Have a remote command execute feature

## compile
Written in rust, so you need install rust  
https://www.rust-lang.org/tools/install

The `server` can't run in windows, I test failed  
The `agent` can running in windows, I tested, but linux I don't know

Just change server ip in `agent/src/main.rs 8:37`  
Then compiling both project  
Run server with sudo

```shell
sudo ./server
```

Run agent with administrator privilege

Enjoy shell!