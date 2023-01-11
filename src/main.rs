#[derive(Default, Debug)]
struct Request {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Default, Debug)]
struct RequestBuilderRef {
    a: i32,
    b: i32,
    c: i32,
}

impl RequestBuilderRef {
    pub fn with_a(&mut self, value: i32) -> &mut Self {
        self.a = value;
        self
    }

    pub fn with_c(&mut self, value: i32) -> &mut Self {
        self.b = value;
        self
    }

    pub fn with_b(&mut self, value: i32) -> &mut Self {
        self.c = value;
        self
    }

    pub fn build(&mut self) -> Request {
        Request {
            a: self.a,
            b: self.b,
            c: self.c,
        }
    }
}

#[derive(Default, Debug)]
struct RequestBuilderOwn {
    a: i32,
    b: i32,
    c: i32,
}

impl RequestBuilderOwn {
    pub fn with_a(mut self, value: i32) -> Self {
        self.a = value;
        self
    }

    pub fn with_c(mut self, value: i32) -> Self {
        self.b = value;
        self
    }

    pub fn with_b(mut self, value: i32) -> Self {
        self.c = value;
        self
    }

    pub fn build(self) -> Request {
        Request {
            a: self.a,
            b: self.b,
            c: self.c,
        }
    }
}

fn main() {
    let mut request_builder = RequestBuilderRef::default();
    request_builder.with_a(10); // use ref to mutate request_builder
    request_builder.with_b(10); // use ref to mutate request_builder
    let request: Request = request_builder.build();
    println!("{request:#?}");

    let request_builder = RequestBuilderOwn::default().with_a(5);
    let request_builder = request_builder.with_b(10); // re-assign to mutate request_builder
    let request = request_builder.build();
    println!("{request:#?}");
}
