

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let a = Some(43);
        match a {
            Some(43) => println!("ok"),
            _ => (),
        }
        if let Some(43) = a {
            println!(" is  ok");
        }
        let x: i32 = 2;
        let b: Option<i32> = Some(4);
        let c = x + b.unwrap();
        println!("{}", c);
    }

    #[test]
    fn test2() {
        let version = [1, 2, 3, 4];
        let t: Result<i32, i32> = if version[0] == 1 { Ok(1) } else { Err(2) };
        println!("{:?}", t);
        // match version[4] {
        //     Ok(v)=> println!("working with version: {:?}", v),
        //     Err(e)=> println!("error parsing header: {:?}", e),
        // }
        // 如果   t 是Err  执行大括号里的代码
        if let Err(e) = t {
            println!("错了 错了");
        };
    }



    fn result_test()->Result<(),std::error::Error>{

        Ok(())
    }


}
