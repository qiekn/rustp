// ----------------------------------------------------------------------------: mod: toy
mod toy {
  // item defualt -> private
  fn private_function() {
    println!("called `toy::private_function()`");
  }

  // use `pub` to change it to public
  pub fn function() {
    println!("called `toy::function()`");
  }

  // 在同一模块中，项可以访问其它项，即使它是私有的。
  pub fn indirect_access() {
    print!("called `toy::indirect_access()`, that\n> ");
    private_function();
  }

  // 模块也可以嵌套
  pub mod nested {
    pub fn function() {
      println!("called `toy::nested::function()`");
    }

    #[allow(dead_code)]
    fn private_function() {
      println!("called `toy::nested::private_function()`");
    }

    // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
    // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
    pub(in crate::toy) fn public_function_in_toy() {
      print!("called `toy::nested::public_function_in_toy_mod()`, that\n > ");
      public_function_in_nested()
    }

    // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
    pub(self) fn public_function_in_nested() {
      println!("called `toy::nested::public_function_in_nested");
    }

    // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
    pub(super) fn public_function_in_super_mod() {
      println!("called toy::nested::public_function_in_super_mod");
    }
  }

  pub fn call_public_function_in_toy() {
    print!("called `toy::call_public_funcion_in_toy_mod()`, that\n> ");
    nested::public_function_in_toy();
    print!("> ");
    nested::public_function_in_super_mod();
  }

  // `pub(crate)` 使得函数只在当前 crate 中可见
  pub(crate) fn public_function_in_crate() {
    println!("called `toy::public_function_in_crate()");
  }

  // 嵌套模块的可见性遵循相同的规则
  mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
      println!("called `toy::private_nested::function()`");
    }
  }
}

fn function() {
  println!("called `function()`");
}

fn main() {
  // 模块机制消除了相同名字的项之间的歧义。
  function();
  toy::function();

  println!("----------------");

  // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
  toy::indirect_access();

  toy::nested::function();
  toy::call_public_function_in_toy();
  println!("----------------");

  // pub(crate) 项可以在同一个 crate 中的任何地方访问
  toy::public_function_in_crate();

  // pub(in path) 项只能在指定的模块中访问
  // 报错！函数 `public_function_in_toy` 是私有的
  // toy::nested::public_function_in_my_mod();
  // 试一试 ^ 取消该行的注释

  // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

  // 报错！`private_function` 是私有的
  // toy::private_function();
  // 试一试 ^ 取消此行注释

  // 报错！`private_function` 是私有的
  //toy::nested::private_function();
  // 试一试 ^ 取消此行的注释

  // Error! `private_nested` is a private module
  //toy::private_nested::function();
  // 试一试 ^ 取消此行的注释
}
