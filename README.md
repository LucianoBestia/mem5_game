# mem5_game

![loc](https://img.shields.io/badge/lines_of_Rust_code-4591-success)
![loc](https://img.shields.io/badge/lines_of_docs/comments-1231-informational)

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web Apps) - part five**  
*Things are changing fast. This is the situation on 2019-12-31.*

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/mem5_game/mem5/index.html>  
The workspace mem5_game is made of projects:  

1. Wasm/WebAssembly (for browsers) frontend - mem5  
2. web server Warp backend - mem5_server  
3. common structures - mem5_common  

Every project has its own readme.md.  

- [mem5/README.md](
https://github.com/LucianoBestia/mem5_game/blob/master/mem5/README.md)  
- [mem5_common/README.md](https://github.com/LucianoBestia/mem5_game/blob/master/mem5_common/README.md)  
- [mem5_server/README.md](https://github.com/LucianoBestia/mem5_game/blob/master/mem5_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/LucianoBestia/mem4_game>  

## Working game server

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/mem5>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make audit` - cargo audit warnings about dependencies  
`cargo make fmt` - format source code  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder  
`cargo make sshadd` - adds identity to ssh-agent for git and publish operations  
`cargo make gitpush` - push the commits to github, uses ssh agent  
`cargo make publish` - publish the webfolder to google vm  
`cargo make udeps` - lists unused dependencies  
`cargo make loc` - Lines Of Rust Code with tokei  
`cargo make depver` - list of not latest dependencies  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  


## TODO and CHANGELOG

Read files [TODO.md](https://github.com/LucianoBestia/mem5_game/blob/master/TODO.md) and [CHANGELOG.md](https://github.com/LucianoBestia/mem5_game/blob/master/CHANGELOG.md).  
