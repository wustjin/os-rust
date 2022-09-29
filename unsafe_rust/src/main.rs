use std::slice;
//rust在编译时都会强制进行内存安全保证
//不安全Rust是用于不安全操作（不会强制执行内存安全保证的）
//五个不安全的超能力
//1.解引用裸指针
fn main() {
    //rust中引用总是有效的
    //而rust中有两个被称为裸指针的类似引用的新类型
    //分可变和不可变：*const T ,*mut T.即解引用后可以直接赋值和不能直接赋值
    //重点：裸指针功能
    //1.允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
    //2.不保证指向有效的内存
    //3.允许为空
    //4.不能实现任何自动清理功能

    //不安全代码中的安全性：unsafe并不会关闭借用检测器和禁止其他Rust安全检查

    //例一：
    let mut num = 20;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //裸指针可以在安全代码中创建，不能在安全代码中解引用

    //例二：
    let address = 0x012345usize;//usize用于确保指针和当前系统指针位数相同
    //i32 -》 u64 无符号补零
    let r = address as *const i32;
    //例三：
    //不能在安全代码中解引用裸指针
    //println!("{}",*r1);
    
    //例四：测试在不安全代码中修改
    unsafe {
        println!("1{}",*r1);
        *r2 = 30;
        println!("2{}",*r1);
    }
    println!("3{}",num);
    //一个主要的应用场景便是调用 C 代码接口

    //例五：裸指针后移
    let mut m = 20;
    let ptr_m = &m as *const i32;
    unsafe{
        ptr_m.offset(20);
    }

    //2.调用不安全函数或者方法
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //使用entern函数调用外部代码
    unsafe{
        println!("C code : {}",abs(-3));
    }
    //3.访问或修改可变静态变量
    //4.实现不安全 trait
    //5.访问联合体中的字段
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32],&mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid<=len);//断言下标下于长度

    //cannot borrow `*slice` as mutable more than once at a time
    //
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len -mid)
        )
    }
}
//FFI,外部函数接口
extern "C" {
    fn abs(input: i32) -> i32;
}