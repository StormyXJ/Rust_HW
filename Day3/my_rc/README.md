# My Rc

主要就是输出的分歧.

* 在添加`Display`这个trait之前,虽然实现了解引用,也只能在变量前添加`*`才能过编译,不过我看ppt上的显示也是这样子使用的?但是和原版Rc有一点不一样.

    ```rust
    println!("{}",*rc_var);
    ```
* 或者使用`#[derive(Debug)]`,如下输出

    ```rust
    println!("{:?}",rc_var);
    ```

* 要是想实现直接print(也就是像原版Rc一样),我只能添加了`Display`,才能直接输出,不过本质还是要添加`*`

    ```rust
    impl<T: std::fmt::Display> fmt::Display for MyRc<T>{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            unsafe{
                write!(f, "{}",*self.data)
            }
            
        }
    }
    ...

    //in main
    println!("{}",rc_var);
    ```