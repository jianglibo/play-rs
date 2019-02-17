#[cfg(test)]
mod tests {

    #[test]
    fn test_on() {
        let v = [1, 2, 3];
        {
            let _vv = &v;
            assert_eq!(_vv.len(), 3);
        }

        // allowed because copied.
        let _v1 = v;
        let _v2 = v;

        // immutable reference, any number is ok.
        let _v3 = &v;
        let _v4 = &v;

        let v = [String::from("a"), String::from("b")];

        let _v1 = v;
        // let _v2 = v; // not allowed, because had moved.

        let v = [String::from("a"), String::from("b")];

        // immutable reference, any number is ok.
        let _v3 = &v;
        let _v4 = &v;

        // iter()是对v的引用，所以x也是对v的元素的引用，下面的代码编译器会提示collect类型错误。不能从Vec<&String> -> Vec<String>
        // let _v5: Vec<String> = v.iter().map(|x| {
        //     x.push('1');
        //     x
        // }).collect();

        // 会提示x是只读，不能改变。
        // let _v5: Vec<&String> = v.iter().map(|x| {
        //     x.push('1');
        //     x
        // }).collect();

        let _v5: Vec<String> = v.iter().map(|x| {
            format!("{}{}", x, "1")
        }).collect();

        let _v5: Vec<String> = v.iter().map(|x| {
            let mut xx = x.clone();
            xx.push('1');
            xx
        }).collect();

        assert_eq!(vec![String::from("a1"), String::from("b1")], _v5);

        println!("{:?}", v); // v still is alive.

        let _v5: Vec<String> = v.into_iter().map(|x| {
            format!("{}{}", x, "1")
        }).collect();

        println!("{:?}", v); // v still is alive.

        // 可更改借用。map的同时改变v。
        let mut v = [String::from("a"), String::from("b")];
        let _v5: Vec<String> = (&mut v).into_iter().map(|x| {
            *x = String::from("123");
            // let _c: Vec<&String> = v.iter().map(|y|y).collect();
            x.clone()
        }).collect();

        assert_eq!(vec![String::from("123"), String::from("123")], v);

        let v = [String::from("a"), String::from("b")];
        let _v5: Vec<String> = v.into_iter().map(|x| {
            x.clone()
        }).collect();

        println!("{:?}", v); // v still is alive.

        let array = [1, 2, 3];
        let _v5: Vec<isize> = array.into_iter().map(|x|{ // x is &i32
            x.clone()
            }).collect();

        let array = &[1, 2, 3];
        let _v5: Vec<isize> = array.into_iter().map(|x|{ // x is &i32
            x.clone()
            }).collect();

        let mut array = &[1, 2, 3];
        let _v5: Vec<isize> = (&mut array).into_iter().map(|x|{ // x is &i32, into_iter know the context.
            x.clone()
            }).collect();

    }
}
