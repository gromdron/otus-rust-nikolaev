trait Receipt {
    fn cook(&self);
}

pub struct Pizza;

impl Receipt for Pizza {
    fn cook(&self) {
        println!("Preparing the base for pizza");
    }
}

pub struct PizzaWithFilling {
    base: Box<Pizza>,
    filling: String
}

impl PizzaWithFilling {
    fn new( base: Box<Pizza>, filling: String ) -> Self {
        Self {
            base,
            filling
        }
    }
}

impl Receipt for PizzaWithFilling {
    fn cook(&self) {
        self.base.cook();
        println!("Add {:?}", self.filling);
    }
}


struct PizzaMaker;

impl PizzaMaker {
    fn make(&self, something_to_cook: Box<dyn Receipt>) {
        something_to_cook.cook();
        println!("Put it in the oven");
    }
}

fn main()
{
    let chef = PizzaMaker {};
    let pizza = Pizza {};
    chef.make( Box::new(pizza) );

    let pepperoni_pizza = PizzaWithFilling::new(
        Box::new(Pizza {}),
        "Pepperoni".to_string()
    );
     chef.make( Box::new(pepperoni_pizza) );

}
