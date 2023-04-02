pub mod server {
    use anyhow::anyhow;
    use etherparse::Ipv4Header;
    use socket2::{SockAddr, Socket};
    use std::io::stdin;
    use std::mem::MaybeUninit;

    pub trait Parse {
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

    pub struct Server {
        pub agent: SockAddr,
        pub sock: Socket,
    }

    impl Server {
        pub fn syc_read(&self) -> anyhow::Result<()> {
            self.sock.send_to(b"syc", &self.agent)?;

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
            self.sock.send_to(b"online", &self.agent)?;
            Ok(())
        }

        pub fn shutdown(&self) -> anyhow::Result<()> {
            self.sock.send_to(b"shutdown", &self.agent)?;
            self.syc_write()?;
            Ok(())
        }

        pub fn handle(&self) -> anyhow::Result<()> {
            loop {
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                let input = input.trim();

                if input.len() == 0 {
                    continue;
                }

                if input == "q" {
                    self.shutdown()?;
                    break;
                }

                self.sock.send_to(input.as_bytes(), &self.agent)?;
                self.syc_write()?;

                let mut data = Box::new([MaybeUninit::new(0u8); 65535]);
                let (n, _) = self.sock.recv_from(&mut *data)?;
                self.syc_read()?;

                let data = data.parse(n)?;
                println!("{}", String::from_utf8_lossy(data.as_slice()));
            }

            Ok(())
        }
    }
}
