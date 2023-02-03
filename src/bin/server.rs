use ws::listen;

fn main() {
    println!("running server");

    let addr = "127.0.0.1:3012";

    listen(addr, |out| {
        let handler = move |msg| {
            let response = format!("{}-{}", msg, "I'm server.".to_string());
            println!("{}", response);
            out.broadcast(response)
        };
        handler
    })
    .unwrap()
}
