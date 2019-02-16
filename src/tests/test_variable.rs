#[cfg(test)]
mod tests {

    #[test]
    fn test_reuse_primary() {
        let v = vec![1,2,3];
        let _vlen1 = v.len();
        let _vlen2 = v.len();

        v.iter().for_each(|x|println!("{}", x));
        v.iter().for_each(|x|println!("{}", x));

        let _v1 = v.iter().map(|x|x);
        let _v2 = v.iter().map(|x|x);

        let _v1: Vec<(isize, isize)> = v.iter().map(|x|{
            (*x, 0)
        }).collect();

        println!("{:?}", v);

        let _v1: Vec<(isize, isize)> = v.iter().map(|x|{
            (*x, 0)
        }).collect();
        println!("{:?}", v);
    }

    #[test]
    fn test_reuse_string() {
        let v: Vec<String> = vec!["1".to_owned(),"2".to_owned(),"3".to_owned()];

        let _vlen1 = v.len();
        let _vlen2 = v.len();

        v.iter().for_each(|x|println!("{}", x));
        v.iter().for_each(|x|println!("{}", x));

        let _v1 = v.iter().map(|x|x);
        let _v2 = v.iter().map(|x|x);

        let v: Vec<String> = vec!["1".to_owned(),"2".to_owned(),"3".to_owned()];
        // cannot move out of borrowed content, 想要改变借用的对象。但是iter()是对v的引用，必须用into_iter才行。
        // let v1: Vec<(String, String)> = v.iter().map(|x|{
        //     (*x, String::from("abc"))
        // }).collect();
        let _v1: Vec<(isize, String)> = v.into_iter().map(|x|{ //变量只能被借用（消耗）一次。
            (0isize, x)
        }).collect();

        // v has used because of into_iter.
        // println!("{:?}", v); //borrow of moved value.

        // let v1: Vec<(isize, String)> = v.iter().map(|x|{
        //     (0isize, *x)
        // }).collect();
        // println!("{:?}", v);
    }

}