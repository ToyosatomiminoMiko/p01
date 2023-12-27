use std::{
    env, fs,
    io::{self, Error, Write},
    str::Chars,
    thread,
    time::Duration,
};

macro_rules! printf {
    /*
    标准输出中的这种行缓冲机制
    在遇到换行符之前，输出的内容并不会隐式的刷新
    这就导致 print! 宏和 println! 宏实际上并不完全相同
     */
    ($($arg:tt)*) =>{
        print!($($arg)*);
        io::stdout().flush().unwrap();
    }
}

fn tnop(s: &str, t: u64) {
    let mut chars: Chars = s.chars();
    loop {
        //printf!("{}", chars.next().unwrap());
        let char: Option<char> = chars.next();
        match char {
            Some(char) => {
                printf!("{}", char);
                thread::sleep(Duration::from_millis(t));
            }
            None => break,
        }
    }
}

fn string_to_static_str(s: String) -> &'static str {
    // 将`String`转换为`&str`
    Box::leak(s.into_boxed_str())
}

/*
```shell
cargo run ./text 250
```
*/
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect(); // 接收命令行传入
    let path: &str = string_to_static_str(args[1].to_string()); // 参数解析 文件路径
    let time: u64 = args[2].to_string().parse::<u64>().unwrap(); // 参数解析 间隔时间
    let content: Vec<u8> = fs::read(path).unwrap(); //打开文件流
    let text: &str = std::str::from_utf8(&content).unwrap(); // 转换为str
    tnop(text, time);
    Ok(())
}
