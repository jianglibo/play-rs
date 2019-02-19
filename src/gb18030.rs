// 从Unicode编码到GB18030编码的映射方法如下：
// U=Unicode编码-0x10000
// m1=U/12600
// n1=U%12600
// m2=n1/1260
// n2=n1%1260
// m3=n2/10
// n3=n2%10
// 第一字节b1=m1+0x90
// 第二字节b2=m2+0x30
// 第三字节b3=m3+0x81
// 第四字节b4=n3+0x30

// pub fn code_point_to_gb18030<T: AsRef<u32>>(cp: T) -> Vec<u8> {
pub fn code_point_to_gb18030_4b(cp: u32) -> Vec<u8> {
    let tmp = cp - 0x1_0000;
    let m1 = (tmp / 12600) as u8;
    let n1 = tmp % 12600;
    let m2 = (n1 / 1260) as u8;
    let n2 = n1 % 1260;
    let m3 = (n2 / 10) as u8;
    let n3 = (n2 % 10) as u8;
    vec![m1 + 0x90, m2 + 0x30, m3 + 0x81, n3 + 0x30]
}



// 从GB18030编码到Unicode编码的映射方法如下：
// 设GB18030编码的四个字节依次为：b1、b2、b3、b4，则
// Unicode编码=0x10000+(b1-0x90)*12600+(b2-0x30)*1260+(b3-0x81)*10+b4-0x30
pub fn gb18030_4b_to_code_point<T: AsRef<[u8]>>(gb4: T) -> Option<u32> {
    let gb4_1 = gb4.as_ref();
    match gb4_1.len() {
        4 => {
            let (b1, b2, b3, b4) = (gb4_1[0],gb4_1[1],gb4_1[2],gb4_1[3]);
            Some(0x10000
                + ((b1-0x90) as u32) * 12600
                + ((b2-0x30) as u32) * 1260
                + ((b3-0x81) as u32) * 10
                + ((b4-0x30) as u32)
            )},
        _ => None
    }
}

pub fn gb18030_2b_to_code_point<T: AsRef<[u8]>>(gb2: T) -> Option<u32> {
    let gb2_1 = gb2.as_ref();
    match gb2_1.len() {
        2 => {
            let (b1, b2, b3, b4) = (gb4_1[0],gb4_1[1],gb4_1[2],gb4_1[3]);
            Some(0x10000
                + ((b1-0x90) as u32) * 12600
                + ((b2-0x30) as u32) * 1260
                + ((b3-0x81) as u32) * 10
                + ((b4-0x30) as u32)
            )},
        _ => None
    }
}