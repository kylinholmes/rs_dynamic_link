# Dynamic link and Load Library in rust

## Demo structure
```bash
├── abi                     # runtime loading library
│  ├── Cargo.toml           # specifies dependent libraries to be used for dynamic linking
│  └── src
│     ├── import.rs         # import impls to this crate
│     ├── lib.rs            # 
│     └── main.rs           # test main: loading abi and binding interface
├── impls                   # dynamic link library 
│  └── src
│     └── lib.rs            # impls
└── README.md
```

