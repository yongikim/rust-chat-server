use ws::connect;
use ws::Message;

fn main() {
    let addr = "ws://127.0.0.1:3012";
    connect(addr, |out| {
        out.send("Hello WebSocket").unwrap();

        // Handlerに型強制される．
        // F: Fn(Message) -> Result<()> に対しては，on_messageのデフォルト実装で
        // 関数自身が実行される
        let handler = move |msg: Message| {
            println!("got message: {}", msg);
            Ok(())
        };

        handler
    })
    .unwrap()
}
