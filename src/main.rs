use oop1::Draw;
use oop1::{Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // セレクトボックスを実際に描画するコード
    }
    
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox{
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button{
                width: 50,
                height: 10m,
                label: String::from("Ok"),

            }),
        ]
    };

    screen.run();
}
