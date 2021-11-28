#[cfg(test)]

mod hallo {
    fn say(input: &str) {
        let var = String::from(input);
        println!("hallo");
    }
}
mod markdown {
    pub enum line {
        h1(String),
        h2(String),
        h3(String),
        h4(String),
        h5(String),
        h6(String),
        p(String),
    }
    impl line {
        pub fn from_md(input: &str) -> line {
            let first_word = line::get_first_word(input);
            match first_word {
                "#" => return line::h1(String::from(line::removefirst(input))),
                "##" => return line::h2(String::from(line::removefirst(input))),
                "###" => return line::h3(String::from(line::removefirst(input))),
                "####" => return line::h4(String::from(line::removefirst(input))),
                "#####" => return line::h5(String::from(line::removefirst(input))),
                "######" => return line::h6(String::from(line::removefirst(input))),
                _ => return line::p(String::from(input)),
            }
        }
        pub fn get_first_word(input: &str) -> &str {
            let mut i = 0;
            for c in input.chars() {
                if c == ' ' {
                    //println!("got a space @ {}", i);
                    return &input[0..i];
                }
                i = i + 1;
            }
            return &input;
        }

        pub fn removefirst(input: &str) -> &str {
            let mut i = 0;
            for c in input.chars() {
                if c == ' ' {
                    //println!("got a space @ {}", i);
                    //println!("returning {}", &input[i+1..]);
                    return &input[i + 1..];
                }
                i = i + 1;
            }
            return &input;
        }
        pub fn as_html(self) -> String {
            match self {
                line::h1(content) => return String::from("<h1>") + &content + "</h1>",
                line::h2(content) => return String::from("<h2>") + &content + "</h2>",
                line::h3(content) => return String::from("<h3>") + &content + "</h3>",
                line::h4(content) => return String::from("<h4>") + &content + "</h4>",
                line::h5(content) => return String::from("<h5>") + &content + "</h5>",
                line::h6(content) => return String::from("<h6>") + &content + "</h6>",
                line::p(content) => return String::from("<p>") + &content + "</p>",
                _ => return String::from(""),
            }
        }
        pub fn as_md(self) -> String {
            match self {
                line::h1(content) => return String::from("# ") + &content,
                line::h2(content) => return String::from("## ") + &content,
                line::h3(content) => return String::from("### ") + &content,
                line::h4(content) => return String::from("#### ") + &content,
                line::h5(content) => return String::from("##### ") + &content,
                line::h6(content) => return String::from("###### ") + &content,
                line::p(content) => return String::from("") + &content,
                _ => return String::from(""),
            }
        }
    }
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(5, 5);
    }
    use super::markdown::*;
    #[test]
    fn h1() {
        //from_md
        let x = line::from_md("# hallo");
        match x {
            line::h1(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h1(String::from("test")).as_html().as_str() {
            "<h1>test</h1>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h1(String::from("this is a test")).as_md().as_str() {
            "# this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn h2() {
        let x = line::from_md("## hallo");
        match x {
            line::h2(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h2(String::from("test")).as_html().as_str() {
            "<h2>test</h2>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h2(String::from("this is a test")).as_md().as_str() {
            "## this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn h3() {
        let x = line::from_md("### hallo");
        match x {
            line::h3(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h3(String::from("test")).as_html().as_str() {
            "<h3>test</h3>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h3(String::from("this is a test")).as_md().as_str() {
            "### this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn h4() {
        let x = line::from_md("#### hallo");
        match x {
            line::h4(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h4(String::from("test")).as_html().as_str() {
            "<h4>test</h4>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h4(String::from("this is a test")).as_md().as_str() {
            "#### this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn h5() {
        let x = line::from_md("##### hallo");
        match x {
            line::h5(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h5(String::from("test")).as_html().as_str() {
            "<h5>test</h5>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h5(String::from("this is a test")).as_md().as_str() {
            "##### this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn h6() {
        let x = line::from_md("###### hallo");
        match x {
            line::h6(val) => {
                if val == String::from("hallo") {
                    println!("okay");
                } else {
                    panic!("can't detect")
                }
            }
            _ => panic!("can't detect"),
        };
        //html
        match line::h6(String::from("test")).as_html().as_str() {
            "<h6>test</h6>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::h6(String::from("this is a test")).as_md().as_str() {
            "###### this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn p() {
        let x = line::from_md("hallo dies ist ein test");
        match x {
            line::p(val) => println!("p"),
            _ => panic!("can't detect"),
        };
        //html
        match line::p(String::from("test")).as_html().as_str() {
            "<p>test</p>" => println!("html good"),
            _ => panic!("html not good"),
        }
        //to_md
        match line::p(String::from("this is a test")).as_md().as_str() {
            "this is a test" => println!("html good"),
            _ => panic!("html not good"),
        }
    }
    #[test]
    fn first_word() {
        let y = "hallo ich bin es";
        let x = line::get_first_word(&y);
        println!("first_word of: {}  ", y);
        println!("is: {}", x);
    }
}
